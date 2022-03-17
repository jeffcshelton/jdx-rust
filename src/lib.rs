mod bindings;
mod dataset;
mod header;

pub mod jdx {
	pub use crate::dataset::*;
	pub use crate::header::*;

	use std::{error, fmt, result};
	use crate::bindings;

	#[derive(Debug, Clone)]
	pub enum Error {
		OpenFile(String),
		CloseFile(String),
		ReadFile(String),
		WriteFile(String),
		CorruptFile(String),

		MemoryFailure,

		UnequalWidths,
		UnequalHeights,
		UnequalBitDepths,
	}

	impl Error {
		pub fn new_with_path<S: Into<String>>(error: bindings::JDXError, path: S) -> Option<Self> {
			let path = path.into();

			match error {
				bindings::JDXError::None => None,
				bindings::JDXError::OpenFile => Some(Error::OpenFile(path)),
				bindings::JDXError::CloseFile => Some(Error::CloseFile(path)),
				bindings::JDXError::ReadFile => Some(Error::ReadFile(path)),
				bindings::JDXError::WriteFile => Some(Error::WriteFile(path)),
				bindings::JDXError::CorruptFile => Some(Error::CorruptFile(path)),
				bindings::JDXError::MemoryFailure => Some(Error::MemoryFailure),
				bindings::JDXError::UnequalWidths => Some(Error::UnequalWidths),
				bindings::JDXError::UnequalHeights => Some(Error::UnequalHeights),
				bindings::JDXError::UnequalBitDepths => Some(Error::UnequalBitDepths),
			}
		}
	}

	impl fmt::Display for Error {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			match self {
				Self::OpenFile(path) => write!(f, "Failed to open file: '{}'", path),
				Self::CloseFile(path) => write!(f, "Failed to close file: '{}'", path),
				Self::ReadFile(path) => write!(f, "Failed to read file: '{}'", path),
				Self::WriteFile(path) => write!(f, "Failed to write to file: '{}'", path),
				Self::CorruptFile(path) => write!(f, "Failed to parse corrupted file: '{}'", path),
				Self::MemoryFailure => write!(f, "A memory failure has occurred."),
				Self::UnequalWidths => write!(f, "Datasets have unequal image widths."),
				Self::UnequalHeights => write!(f, "Datasets have unequal image heights."),
				Self::UnequalBitDepths => write!(f, "Datasets have unequal bit depths."),
			}
		}
	}

	impl error::Error for Error {}

	pub type Result<T> = result::Result<T, Error>;
}
