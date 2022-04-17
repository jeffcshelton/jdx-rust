use crate::{ffi, Header};
use std::slice;

#[derive(Clone)]
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
				image.bit_depth as usize;

			let raw_data = slice::from_raw_parts_mut(image.raw_data, data_size).to_vec();

			let label = std::ffi::CStr::from_ptr(image.label as *mut i8)
				.to_str()
				.unwrap()
				.to_owned();

			return Self {
				raw_data: raw_data,
				width: image.width,
				height: image.height,
				bit_depth: image.bit_depth,
				label: label,
				label_index: image.label_index,
			};
		}
	}
}

pub struct ImageIterator<'a> {
	pub(crate) header: &'a Header,

	pub(crate) index: usize,
	pub(crate) image_data: &'a [u8],
	pub(crate) label_data: &'a [u16],
}

impl<'a> Iterator for ImageIterator<'a> {
	type Item = Image;

	fn next(&mut self) -> Option<Image> {
		if self.index >= self.header.image_count as usize {
			return None;
		}
		
		let image_size = self.header.image_size();
		let start_data = self.index * image_size;
		let end_data = start_data + image_size;

		let raw_data = self.image_data[start_data..end_data].to_vec();
		let label_index = self.label_data[self.index];

		self.index += 1;

		return Some(Image {
			raw_data: raw_data,
			width: self.header.image_width,
			height: self.header.image_height,
			bit_depth: self.header.bit_depth,
			label: self.header.labels[label_index as usize].clone(),
			label_index: label_index,
		});
	}
}
