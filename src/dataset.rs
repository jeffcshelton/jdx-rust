use std::{
	fs::File,
	io::{Read, Write},
	mem,
	path::Path,
	slice::{Iter, IterMut}, collections::HashMap,
};

use flate2::{
	Compression,
	read::DeflateDecoder,
	write::DeflateEncoder,
};

use crate::{
	Error,
	Header,
	Label,
	Result,
};

pub type LabeledImage = (Box<[u8]>, Label);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dataset {
	header: Header,
	images: Vec<LabeledImage>,
}

impl Dataset {
	pub fn read_from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
		Self::read_from_file(&mut File::open(path)?)
	}

	pub fn read_from_file(file: &mut File) -> Result<Self> {
		let header = Header::read_from_file(file)?;

		let mut body_size_bytes = [0_u8; 8];
		file.read_exact(&mut body_size_bytes)?;

		let body_size = usize::try_from(
			u64::from_le_bytes(body_size_bytes)
		).unwrap();

		let mut decoder = DeflateDecoder::new(file);
		let mut decompressed_data = Vec::with_capacity(body_size);
		decoder.read_to_end(&mut decompressed_data)?;

		let image_size = header.image_size();
		let label_size = mem::size_of::<Label>();

		let images = decompressed_data
			.chunks_exact(image_size + label_size)
			.map(|chunk| {
				let image_data = Box::<[u8]>::from(&chunk[..image_size]);
				let label = Label::from_le_bytes(
					chunk[image_size..]
						.try_into()
						.unwrap()
				);

				(image_data, label)
			})
			.collect();

		for &(_, label) in &images {
			if usize::from(label) > header.classes.len() {
				return Err(Error::CorruptFile);
			}
		}

		Ok(Self {
			header: header,
			images: images,
		})
	}

	pub fn with_header(header: Header) -> Self {
		Self {
			header: header,
			images: Vec::new(),
		}
	}
}

impl Dataset {
	#[inline]
	pub fn get(&self, index: usize) -> Option<&LabeledImage> {
		self.images.get(index)
	}

	#[inline]
	pub fn get_mut(&mut self, index: usize) -> Option<&mut LabeledImage> {
		self.images.get_mut(index)
	}

	#[inline]
	pub fn get_header(&self) -> &Header {
		&self.header
	}

	#[inline]
	pub fn iter(&self) -> Iter<LabeledImage> {
		self.images.iter()
	}

	#[inline]
	pub fn iter_mut(&mut self) -> IterMut<LabeledImage> {
		self.images.iter_mut()
	}

	pub fn append(&mut self, mut other: Dataset) -> Result<()> {
		if !self.header.is_compatible_with(&other.header) {
			return Err(Error::IncompatibleDimensions);
		}

		let other_classes = other.header.classes.clone();
		let mut label_map = HashMap::<u16, u16>::new();

		// This loop merges labels in the 'other' Dataset into self
		for (_, image_label) in other.iter_mut() {
			if let Some(mapped_label) = label_map.get(image_label) {
				*image_label = *mapped_label;
			} else {
				let class_name = other_classes
					.get(usize::from(*image_label))
					.unwrap();

				let mapped_label = self.header.classes
					.iter()
					.position(|s| s == class_name)
					.unwrap_or_else(|| {
						self.header.classes.push(class_name.to_owned());
						self.header.classes.len() - 1
					});

				if let Ok(mapped_label) = u16::try_from(mapped_label) {
					label_map.insert(*image_label, mapped_label);
					*image_label = mapped_label;
				} else {
					self.header.classes.pop();
					return Err(Error::ClassLimitExceeded);
				}
			}
		}

		self.header.image_count += other.header.image_count;
		self.images.append(&mut other.images);

		Ok(())
	}

	pub fn extend(&mut self, other: &Dataset) -> Result<()> {
		self.append(other.clone())
	}

	pub fn push(&mut self, raw_image: Box<[u8]>, class_name: &str) -> Result<()> {
		if raw_image.len() != self.header.image_size() {
			return Err(Error::IncompatibleDimensions);
		} else if class_name.len() + 1 > u16::MAX.into() {
			return Err(Error::ClassLengthLimitExceeded);
		}

		let mapped_label = self.header.classes
			.iter()
			.position(|s| s == class_name)
			.unwrap_or_else(|| {
				self.header.classes.push(class_name.to_owned());
				self.header.classes.len() - 1
			});

		if let Ok(mapped_label) = u16::try_from(mapped_label) {
			self.header.image_count += 1;
			self.images.push((raw_image, mapped_label));
		} else {
			self.header.classes.pop();
			return Err(Error::ClassLimitExceeded);
		}

		Ok(())
	}

	pub fn write_to_path<P: AsRef<Path>>(&self, path: P) -> Result<()> {
		self.write_to_file(&mut File::create(path)?)
	}

	pub fn write_to_file(&self, file: &mut File) -> Result<()> {
		self.header.write_to_file(file)?;

		let mut compressed_buffer = Vec::new();
		let mut encoder = DeflateEncoder::new(&mut compressed_buffer, Compression::new(9));

		for (image_data, label) in &self.images {
			encoder.write_all(&image_data)?;
			encoder.write_all(&label.to_le_bytes())?;
		}

		drop(encoder);

		file.write_all(&u64::try_from(compressed_buffer.len()).unwrap().to_le_bytes())?;
		file.write_all(&compressed_buffer)?;

		Ok(())
	}
}
