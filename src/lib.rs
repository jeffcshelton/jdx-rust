pub mod version;
pub use version::Version;

pub mod dataset;
pub use dataset::{
	Dataset,
	LabeledImage,
};

pub mod header;
pub use header::Header;

#[cfg(test)]
mod tests;

use std::{error, fmt, result, io};

pub type Label = u16;

#[derive(Debug, Clone)]
pub enum Error {
	Io(io::ErrorKind),
	CorruptFile,
	UnrecognizedVersion(u8),
	IncompatibleDimensions,

	ClassLimitExceeded,
	ClassLengthLimitExceeded,
}

impl From<io::Error> for Error {
	fn from(io_error: io::Error) -> Self {
		Self::Io(io_error.kind())
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Io(error_kind) => write!(f, "Encountered IO error: {error_kind}."),
			Self::CorruptFile => write!(f, "Failed to parse corrupted file."),
			Self::UnrecognizedVersion(ver) => write!(f, "Unrecognized version number: '{ver}'. An update may be required to read this file."),
			Self::IncompatibleDimensions => write!(f, "Failed to merge due to incompatible dimensions."),
			Self::ClassLimitExceeded => write!(f, "The number of classes in a dataset cannot exceed 65,535."),
			Self::ClassLengthLimitExceeded => write!(f, "The number of bytes in a class name cannot exceed 65,534."),
		}
	}
}

impl error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;
