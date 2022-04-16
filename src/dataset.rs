use crate::{bindings, jdx};
use std::{slice, ffi};

#[derive(Clone)]
pub struct Dataset {
	pub header: jdx::Header,
	pub items: Vec<Item>,
}

impl Dataset {
	pub fn read_from_path<S: Into<String>>(path: S) -> jdx::Result<Self> {
		let path_string = path.into();
		let path_cstring = ffi::CString::new(path_string.clone()).unwrap();
		let dataset_ptr = unsafe { bindings::JDX_AllocDataset() };

		let read_error = unsafe {
			bindings::JDX_ReadDatasetFromPath(dataset_ptr, path_cstring.as_ptr())
		};

		let result = match jdx::Error::new_with_path(read_error, path_string) {
			Some(error) => Err(error),
			None => Ok(unsafe { &*dataset_ptr }.into())
		};

		unsafe {
			bindings::JDX_FreeDataset(dataset_ptr);
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

impl From<&bindings::JDXDataset> for Dataset {
	fn from(dataset: &bindings::JDXDataset) -> Self {
		let items = unsafe {
			slice::from_raw_parts(dataset.items, (*dataset.header).item_count as usize)
				.iter()
				.map(|item| item.into())
				.collect()
		};

		Dataset {
			header: unsafe { &*dataset.header }.into() ,
			items: items,
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
