pub mod version;
pub use version::Version;

pub mod dataset;
pub use dataset::{
	Dataset,
	ImgIterator,
};

pub mod header;
pub use header::Header;

pub mod image;
pub use image::Image;

#[cfg(test)]
mod tests;

use std::{error, fmt, result, io};

pub type Label = u16;

#[derive(Debug, Clone)]
pub enum Error {
	Io(io::ErrorKind),
	CorruptFile,
	IncompatibleHeaders,
}

impl From<io::Error> for Error {
	fn from(io_error: io::Error) -> Self {
		Self::Io(io_error.kind())
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Io(error_kind) => write!(f, "Encountered IO error '{}'", error_kind),
			Self::CorruptFile => write!(f, "Failed to parse corrupted file."),
			Self::IncompatibleHeaders => write!(f, "Failed to merge due to incompatible headers."),
		}
	}
}

impl error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;
