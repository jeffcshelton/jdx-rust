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
