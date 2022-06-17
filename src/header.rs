use std::{fs::File, path::Path, io::{Read, Write}, ops::Add};
use crate::{Error, Result, Version};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Header {
	pub version: Version,

	pub image_width: u16,
	pub image_height: u16,
	pub bit_depth: u8,
	pub image_count: usize,

	pub labels: Vec<String>,
}

impl Header {
	#[inline]
	pub fn image_size(&self) -> usize {
		usize::from(self.image_width) *
		usize::from(self.image_height) *
		usize::from(self.bit_depth / 8)
	}

	pub fn read_from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
		Self::read_from_file(&mut File::open(path)?)
	}

	pub fn read_from_file(file: &mut File) -> Result<Self> {
		let mut corruption_check = [0x00; 3];
		file.read_exact(&mut corruption_check)?;

		if &corruption_check != b"JDX" { // Corresponds to "JDX"
			return Err(Error::CorruptFile);
		}

		let version = Version::read_from_file(file)?;

		let mut raw_buffer = [0x00; 17]; // TODO: Rename better
		file.read_exact(&mut raw_buffer)?;

		let label_bytes = u32::from_le_bytes(
			raw_buffer[5..9]
				.try_into()
				.unwrap()
		);

		let mut raw_labels = vec![0_u8; usize::try_from(label_bytes).unwrap()];
		file.read_exact(&mut raw_labels)?;

		// TODO: Add check & filter for zero-length strings
		let labels = raw_labels
			.split(|&byte| byte == 0)
			.filter_map(|byte_str| std::str::from_utf8(byte_str).ok())
			.map(str::to_owned)
			.filter(|label| !label.is_empty())
			.collect();

		return Ok(Self {
			version: version,
			image_width: u16::from_le_bytes(raw_buffer[0..2].try_into().unwrap()),
			image_height: u16::from_le_bytes(raw_buffer[2..4].try_into().unwrap()),
			bit_depth: raw_buffer[4],
			image_count: u64::from_le_bytes(raw_buffer[9..17].try_into().unwrap()).try_into().unwrap(),
			labels: labels,
		});
	}
}

impl Header {
	pub fn is_compatible_with(&self, other: &Header) -> bool {
		self.image_width == other.image_width
		&& self.image_height == other.image_height
		&& self.bit_depth == other.bit_depth
	}

	pub fn write_to_file(&self, file: &mut File) -> Result<()> {
		file.write_all(b"JDX")?;
		self.version.write_to_file(file)?;

		let label_bytes = self.labels
			.iter()
			.map(String::len)
			.sum::<usize>()
			.add(self.labels.len());

		file.write_all(&self.image_width.to_le_bytes())?;
		file.write_all(&self.image_height.to_le_bytes())?;
		file.write_all(&self.bit_depth.to_le_bytes())?;
		file.write_all(&u32::try_from(label_bytes).unwrap().to_le_bytes())?;
		file.write_all(&u64::try_from(self.image_count).unwrap().to_le_bytes())?;

		for label in &self.labels {
			file.write_all(label.as_str().as_bytes())?;
			file.write_all(&[0x00])?;
		}

		file.flush()?;
		Ok(())
	}
}
