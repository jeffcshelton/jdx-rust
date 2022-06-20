use std::{fs::File, io::{Read, Write}};
use crate::{Result, Error};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Version {
	V0
}

impl Version {
	pub(crate) fn read_from_file(file: &mut File) -> Result<Self> {
		let mut raw_version = [0_u8; 1];
		file.read_exact(&mut raw_version)?;

		let version = match raw_version[0] {
			0 => Self::V0,
			_ => Err(Error::CorruptFile)?,
		};

		Ok(version)
	}

	pub(crate) fn write_to_file(&self, file: &mut File) -> Result<()> {
		let raw_version: u8 = match self {
			Self::V0 => 0
		};

		file.write_all(&raw_version.to_le_bytes())?;
		Ok(())
	}
}

impl ToString for Version {
	fn to_string(&self) -> String {
		match self {
			Self::V0 => "v0"
		}.to_owned()
	}
}