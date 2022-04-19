use crate::ffi;
use std::slice;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Image {
	pub raw_data: Vec<u8>,

	pub width: u16,
	pub height: u16,
	pub bit_depth: u8,

	pub label: String,
	pub label_index: u16,
}

impl From<*mut ffi::JDXImage> for Image {
	fn from(image_ptr: *mut ffi::JDXImage) -> Self {
		unsafe {
			let image = *image_ptr;

			let data_size =
				image.width as usize *
				image.height as usize *
				image.bit_depth as usize / 8;

			let raw_data = slice::from_raw_parts_mut(image.raw_data, data_size).to_vec();
			let label = std::ffi::CStr::from_ptr(image.label_str as *mut i8)
				.to_str()
				.unwrap()
				.to_owned();

			return Self {
				raw_data: raw_data,
				width: image.width,
				height: image.height,
				bit_depth: image.bit_depth,
				label: label,
				label_index: image.label_num,
			};
		}
	}
}
