use libc::{c_char, c_void};
use std::{slice, mem};
use crate::jdx;

pub type Version = jdx::ffi::JDXVersion;

#[derive(Clone)]
pub struct Header {
	pub version: Version,

	pub image_width: u16,
	pub image_height: u16,
	pub bit_depth: u8,
	pub image_count: u64,

	pub labels: Vec<String>,
}

impl Header {
	pub fn read_from_path<S: Into<String>>(path: S) -> jdx::Result<Self> {
		let path_string = path.into();
		let path_cstring = ffi::CString::new(path_string.clone()).unwrap();
		let header_ptr = unsafe { bindings::JDX_AllocHeader() };

		let read_error = unsafe {
			bindings::JDX_ReadHeaderFromPath(header_ptr, path_cstring.as_ptr())
		};

		let result = match read_error {
			bindings::JDXError::None => unsafe { Ok((&*header_ptr).into()) },
			bindings::JDXError::OpenFile => Err(jdx::Error::OpenFile(path_string)),
			bindings::JDXError::ReadFile => Err(jdx::Error::ReadFile(path_string)),
			bindings::JDXError::CorruptFile => Err(jdx::Error::CorruptFile(path_string)),
			bindings::JDXError::CloseFile => Err(jdx::Error::CloseFile(path_string)),
			_ => Err(jdx::Error::ReadFile(path_string))
		};

		unsafe {
			bindings::JDX_FreeHeader(header_ptr);
		}

		return result;
	}
}

impl From<&bindings::JDXHeader> for Header {
	fn from(header: &bindings::JDXHeader) -> Self {
		let labels = unsafe {
			slice::from_raw_parts(header.labels, header.label_count as usize)
			.iter()
			.map(|&label| ffi::CStr::from_ptr(label).to_string_lossy().into_owned())
			.collect()
		};

		Header {
			version: header.version,
			image_width: header.image_width,
			image_height: header.image_height,
			bit_depth: header.bit_depth,
			labels: labels,
			item_count: header.item_count,
		}
	}
}

impl Version {
	pub fn current() -> Self {
		unsafe { bindings::JDX_VERSION }
	}
}

impl ToString for Version {
	fn to_string(&self) -> String {
		let build_type_str = match self.build_type {
			bindings::JDX_BUILD_DEV => " (dev build)",
			bindings::JDX_BUILD_ALPHA => "-alpha",
			bindings::JDX_BUILD_BETA => "-beta",
			bindings::JDX_BUILD_RC => "-rc",
			bindings::JDX_BUILD_RELEASE => "",
			_ => " (invalid build)"
		};
		
		format!("v{}.{}.{}{}", self.major, self.minor, self.patch, build_type_str)
	}
}
