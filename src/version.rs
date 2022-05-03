use std::{fs::File, path::Path, io::{Read, Write}};
use crate::{Result, Error};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Version {
	pub major: u8,
	pub minor: u8,
	pub patch: u8,
	pub build_type: BuildType,
}

impl Version {
	pub fn read_from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
		Self::read_from_file(&mut File::open(path)?)
	}

	pub fn read_from_file(file: &mut File) -> Result<Self> {
		let mut version_buffer = [0x00; 4];
		file.read_exact(&mut version_buffer)?;

		Ok(Self {
			major: version_buffer[0],
			minor: version_buffer[1],
			patch: version_buffer[2],
			build_type: BuildType::from_u8(version_buffer[3])?
		})
	}
}

impl Version {
	#[inline]
	pub fn current() -> Self {
		Self {
			major: 0,
			minor: 4,
			patch: 0,
			build_type: BuildType::Dev,
		}
	}

	pub fn is_compatible_with(&self, other: Version) -> bool {
		if self.major == 0 {
				self.major == other.major
				&& self.minor == other.minor
		} else {
			self.major == other.major
		}
	}

	pub fn write_to_file(&self, file: &mut File) -> Result<()> {
		file.write_all(&self.major.to_le_bytes())?;
		file.write_all(&self.minor.to_le_bytes())?;
		file.write_all(&self.patch.to_le_bytes())?;
		file.write_all(&self.build_type.to_u8().to_le_bytes())?;

		Ok(())
	}
}

impl ToString for Version {
	fn to_string(&self) -> String {
		format!("v{}.{}.{}{}", self.major, self.minor, self.patch, self.build_type.postfix())
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuildType {
	Dev,
	Alpha,
	Beta,
	Rc,
	Release,
}

impl BuildType {
	pub fn postfix(&self) -> String {
		match self {
			Self::Dev => " (dev build)",
			Self::Alpha => "-alpha",
			Self::Beta => "-beta",
			Self::Rc => "-rc",
			Self::Release => "",
		}.to_owned()
	}

	pub fn to_u8(&self) -> u8 {
		match self {
			Self::Dev => 0,
			Self::Alpha => 1,
			Self::Beta => 2,
			Self::Rc => 3,
			Self::Release => 4,
		}
	}
}

impl BuildType {
	pub fn from_u8(raw: u8) -> Result<Self> {
		Ok(match raw {
			0 => Self::Dev,
			1 => Self::Alpha,
			2 => Self::Beta,
			3 => Self::Rc,
			4 => Self::Release,
			_ => Err(Error::CorruptFile)?,
		})
	}
}
