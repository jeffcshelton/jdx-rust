pub mod version;
pub use version::Version;

pub mod dataset;
pub use dataset::Dataset;

pub mod header;
pub use header::Header;

pub mod image;
pub use image::{
	Image,
	ImageIterator,
};

use std::{error, fmt, result};
pub use libjdx_sys as ffi;
use libc::c_void;

pub type Label = ffi::JDXLabel;

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
	pub fn new_with_path(error: ffi::JDXError, path: &str) -> Option<Self> {
		use ffi::JDXError;

		let path = path.to_owned();

		match error {
			JDXError::None => None,
			JDXError::OpenFile => Some(Self::OpenFile(path)),
			JDXError::CloseFile => Some(Self::CloseFile(path)),
			JDXError::ReadFile => Some(Self::ReadFile(path)),
			JDXError::WriteFile => Some(Self::WriteFile(path)),
			JDXError::CorruptFile => Some(Self::CorruptFile(path)),
			JDXError::MemoryFailure => Some(Self::MemoryFailure),
			JDXError::UnequalWidths => Some(Self::UnequalWidths),
			JDXError::UnequalHeights => Some(Self::UnequalHeights),
			JDXError::UnequalBitDepths => Some(Self::UnequalBitDepths),
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

// TODO: Find a better spot for this function
pub(crate) unsafe fn memdup(src: *const c_void, size: usize) -> *mut c_void {
	let block = libc::malloc(size);
	libc::memcpy(block, src, size);

	return block;
}
