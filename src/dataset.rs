use std::{slice, ptr, mem};
use crate::{Header, ffi};
use libc::c_void;

#[derive(Clone)]
pub struct Dataset {
	pub header: Header,
	image_data: Vec<u8>,
	label_data: Vec<u16>,
}

impl Dataset {
	pub fn read_from_path(path: &str) -> crate::Result<Self> {
		let path_cstring = std::ffi::CString::new(path).unwrap();

		let dataset_ptr = unsafe { ffi::JDX_AllocDataset() };
		let read_error = unsafe { ffi::JDX_ReadDatasetFromPath(dataset_ptr, path_cstring.as_ptr()) };

		if let Some(error) = crate::Error::new_with_path(read_error, path) {
			return Err(error);
		}

		return Ok(dataset_ptr.into());
	}

	pub fn get_image(&self, index: usize) -> Option<Image> {
		unsafe {
			let image_ptr = ffi::JDX_GetImage(
				Into::<*mut ffi::JDXDataset>::into(self),
				index as u64
			);

			if image_ptr.is_null() {
				return None;
			}

			return Some(image_ptr.into());
		}
	}
}

impl From<*mut ffi::JDXDataset> for Dataset {
	fn from(dataset_ptr: *mut ffi::JDXDataset) -> Self {
		unsafe {
			let dataset = *dataset_ptr;

			let header: Header = dataset.header.into();
			(*dataset_ptr).header = ptr::null_mut();

			let image_data = slice::from_raw_parts_mut(
				dataset._raw_image_data,
				ffi::JDX_GetImageSize(dataset.header) * header.image_count as usize,
			).to_vec();

			let label_data = slice::from_raw_parts_mut(
				dataset._raw_labels,
				mem::size_of::<ffi::JDXLabel>() * header.image_count as usize,
			).to_vec();

			ffi::JDX_FreeDataset(dataset_ptr);

			return Self {
				header: header,
				image_data: image_data,
				label_data: label_data,
			};
		}
	}
}

impl From<&Dataset> for *mut ffi::JDXDataset {
	fn from(dataset: &Dataset) -> Self {
		let header_ptr: *mut ffi::JDXHeader = (&dataset.header).into();

		unsafe {
			let dataset_ptr = ffi::JDX_AllocDataset();

			*dataset_ptr = ffi::JDXDataset {
				header: header_ptr,
				_raw_image_data: ffi::memdup(
					dataset.image_data.as_ptr() as *const c_void,
					mem::size_of_val(&dataset.image_data as &[u8]
				)) as *mut u8,
				_raw_labels: ffi::memdup(
					dataset.label_data.as_ptr() as *const c_void,
					mem::size_of_val(&dataset.label_data as &[u16]
				)) as *mut u16,
			};

			return dataset_ptr;
		}
	}
}

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
