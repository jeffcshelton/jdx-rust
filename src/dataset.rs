use libc::c_void;

use crate::jdx::{self, Header};
use std::{slice, ptr, mem};

#[derive(Clone)]
pub struct Dataset {
	pub header: Header,
	image_data: Vec<u8>,
	label_data: Vec<u16>,
}

impl Dataset {
	pub fn read_from_path(path: &str) -> jdx::Result<Self> {
		let path_cstring = std::ffi::CString::new(path).unwrap();

		let dataset_ptr = unsafe { jdx::ffi::JDX_AllocDataset() };
		let read_error = unsafe { jdx::ffi::JDX_ReadDatasetFromPath(dataset_ptr, path_cstring.as_ptr()) };

		if let Some(error) = jdx::Error::new_with_path(read_error, path) {
			return Err(error);
		}

		return Ok(dataset_ptr.into());
	}
}

impl From<*mut jdx::ffi::JDXDataset> for Dataset {
	fn from(dataset_ptr: *mut jdx::ffi::JDXDataset) -> Self {
		unsafe {
			let dataset = *dataset_ptr;

			let header: Header = dataset.header.into();
			(*dataset_ptr).header = ptr::null_mut();

			let image_data = slice::from_raw_parts_mut(
				dataset._raw_image_data,
				jdx::ffi::JDX_GetImageSize(dataset.header) * header.image_count as usize,
			).to_vec();

			let label_data = slice::from_raw_parts_mut(
				dataset._raw_labels,
				mem::size_of::<jdx::ffi::JDXLabel>() * header.image_count as usize,
			).to_vec();

			jdx::ffi::JDX_FreeDataset(dataset_ptr);

			return Self {
				header: header,
				image_data: image_data,
				label_data: label_data,
			};
		}
	}
}

impl From<&Dataset> for *mut jdx::ffi::JDXDataset {
	fn from(dataset: &Dataset) -> Self {
		let header_ptr: *mut jdx::ffi::JDXHeader = (&dataset.header).into();

		unsafe {
			let dataset_ptr = jdx::ffi::JDX_AllocDataset();

			*dataset_ptr = jdx::ffi::JDXDataset {
				header: header_ptr,
				_raw_image_data: jdx::ffi::memdup(
					dataset.image_data.as_ptr() as *const c_void,
					mem::size_of_val(&dataset.image_data as &[u8]
				)) as *mut u8,
				_raw_labels: jdx::ffi::memdup(
					dataset.label_data.as_ptr() as *const c_void,
					mem::size_of_val(&dataset.label_data as &[u16]
				)) as *mut u16,
			};

			return dataset_ptr;
		}
	}
}

#[derive(Clone)]
pub struct Item {
	pub data: Vec<u8>,

	pub width: u16,
	pub height: u16,
	pub bit_depth: u8,

	pub label: jdx::Label,
}

impl From<&bindings::JDXItem> for Item {
	fn from(libjdx_item: &bindings::JDXItem) -> Self {
		let image_size =
			libjdx_item.width as usize *
			libjdx_item.height as usize *
			(libjdx_item.bit_depth / 8) as usize;
		
		let image_data = unsafe {
			slice::from_raw_parts(libjdx_item.data, image_size).to_vec()
		};

		Item {
			data: image_data,
			width: libjdx_item.width,
			height: libjdx_item.height,
			bit_depth: libjdx_item.bit_depth,
			label: libjdx_item.label,
		}
	}
}
