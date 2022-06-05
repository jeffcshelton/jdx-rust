use std::{
	fs::File,
	io::{Read, Write},
	path::Path,
	mem,
};

use flate2::{
	Compression,
	read::DeflateDecoder,
	write::DeflateEncoder,
};

use crate::{
	Error,
	Header,
	Image,
	Label,
	Result,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dataset {
	header: Header,
	raw_data: Vec<u8>,
}

impl Dataset {
	pub fn read_from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
		Self::read_from_file(&mut File::open(path)?)
	}

	pub fn read_from_file(file: &mut File) -> Result<Self> {
		let header = Header::read_from_file(file)?;

		let mut body_size_bytes = [0_u8; 8];
		file.read_exact(&mut body_size_bytes)?;
		let body_size = u64::from_le_bytes(body_size_bytes) as usize;

		let mut decoder = DeflateDecoder::new(file);
		let mut decompressed_data = Vec::with_capacity(body_size);
		decoder.read_to_end(&mut decompressed_data)?;

		Ok(Self {
			header: header,
			raw_data: decompressed_data,
		})
	}

	pub fn with_header(header: Header) -> Self {
		Self {
			header: header,
			raw_data: Vec::new(),
		}
	}
}

impl Dataset {
	#[inline]
	pub fn header(&self) -> &Header {
		&self.header
	}

	#[inline]
	pub fn iter(&self) -> ImgIterator {
		ImgIterator {
			dataset: self,
			index: 0,
		}
	}

	pub fn append(&mut self, mut dataset: Dataset) -> Result<()> {
		if !self.header.is_compatible_with(&dataset.header) {
			return Err(Error::IncompatibleHeaders);
		}

		self.header.image_count += dataset.header.image_count;

		// TODO: Do label correction & add test
		self.raw_data.append(&mut dataset.raw_data);
		Ok(())
	}

	pub fn extend(&mut self, dataset: &Dataset) -> Result<()> {
		if !self.header.is_compatible_with(&dataset.header) {
			return Err(Error::IncompatibleHeaders);
		}

		// TODO: Do label correction & add test
		self.raw_data.extend(dataset.raw_data.iter());
		self.header.image_count += dataset.header.image_count;

		Ok(())
	}

	pub fn get_image(&self, index: usize) -> Option<Image> {
		if index >= self.header.image_count {
			return None;
		}

		let image_size = self.header.image_size();
		let label_size = mem::size_of::<Label>();
		let block_size = image_size + label_size;

		let start_block = index * block_size;
		let end_image = start_block + image_size;
		let end_label = end_image + label_size;

		let image_data = &self.raw_data[start_block..end_image];
		let label_index = Label::from_le_bytes(
			self.raw_data[end_image..end_label]
				.try_into()
				.unwrap()
		);

		Some(Image {
			raw_data: image_data,
			width: self.header.image_width,
			height: self.header.image_height,
			bit_depth: self.header.bit_depth,
			label: self.header.labels.get(label_index as usize).unwrap(),
			label_index: label_index,
		})
	}

	pub fn push(&mut self, image: Image) -> Result<()> {
		if self.header.image_width != image.width
		|| self.header.image_height != image.height
		|| self.header.bit_depth != image.bit_depth {
			return Err(Error::IncompatibleHeaders);
		}

		let label_index = self.header.labels
			.iter()
			.position(|label| label == &image.label)
			.unwrap_or_else(|| {
				self.header.labels.push(image.label.to_owned());
				self.header.labels.len() - 1
			}) as u16; // TODO: Remove as and replace with explicit check

		self.raw_data.append(&mut image.raw_data.to_vec());
		self.raw_data.extend(&label_index.to_le_bytes());
		self.header.image_count += 1;

		Ok(())
	}

	pub fn write_to_path<P: AsRef<Path>>(&self, path: P) -> Result<()> {
		self.write_to_file(&mut File::create(path)?)
	}

	pub fn write_to_file(&self, file: &mut File) -> Result<()> {
		self.header.write_to_file(file)?;

		let mut compressed_buffer = Vec::new();
		DeflateEncoder::new(&mut compressed_buffer, Compression::new(9))
			.write_all(&self.raw_data)?;

		file.write_all(&(compressed_buffer.len() as u64).to_le_bytes())?;
		file.write_all(&compressed_buffer)?;

		Ok(())
	}
}

pub struct ImgIterator<'a> {
	dataset: &'a Dataset,
	index: usize,
}

impl<'a> Iterator for ImgIterator<'a> {
	type Item = Image<'a>;

	fn next(&mut self) -> Option<Image<'a>> {
		self.index += 1;

		self.dataset.get_image(self.index - 1)
	}
}

impl ExactSizeIterator for ImgIterator<'_> {
	fn len(&self) -> usize {
		self.dataset.header.image_count as usize
	}
}
