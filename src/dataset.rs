use crate::{Error, ffi, Header, Image, Result};
use std::{slice, ptr, mem};
use libc::c_void;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dataset {
	header: Header,
	image_data: Vec<u8>,
	label_data: Vec<u16>,
}

impl Dataset {
	pub fn read_from_path(path: &str) -> Result<Self> {
		let path_cstring = std::ffi::CString::new(path).unwrap();

		let dataset_ptr = unsafe { ffi::JDX_AllocDataset() };
		let read_error = unsafe { ffi::JDX_ReadDatasetFromPath(dataset_ptr, path_cstring.as_ptr()) };

		if let Some(error) = Error::new_with_path(read_error, path) {
			return Err(error);
		}

		return Ok(dataset_ptr.into());
	}
}

impl Dataset {
	#[inline]
	pub fn header(&self) -> Header {
		self.header.clone()
	}

	#[inline]
	pub fn iter(&self) -> ImageIterator {
		ImageIterator {
			dataset: self,
			index: 0,
		}
	}

	pub fn get_image(&self, index: usize) -> Option<Image> {
		unsafe {
			let image_ptr = ffi::JDX_GetImage(
				self.into_ptr(),
				index as u64
			);

			if image_ptr.is_null() {
				return None;
			}

			return Some(image_ptr.into());
		}
	}

	pub unsafe fn into_ptr(&self) -> *mut ffi::JDXDataset {
		let header_ptr: *mut ffi::JDXHeader = (&self.header).into();

		let dataset_ptr = ffi::JDX_AllocDataset();

		*dataset_ptr = ffi::JDXDataset {
			header: header_ptr,
			_raw_image_data: crate::memdup(
				self.image_data.as_ptr() as *const c_void,
				mem::size_of_val(&self.image_data as &[u8]
			)) as *mut u8,
			_raw_labels: crate::memdup(
				self.label_data.as_ptr() as *const c_void,
				mem::size_of_val(&self.label_data as &[u16]
			)) as *mut u16,
		};

		return dataset_ptr;
	}

	pub fn write_to_path(&self, path: &str) -> Result<()> {
		let path_cstring = std::ffi::CString::new(path).unwrap();

		let dataset_ptr = unsafe { self.into_ptr() };
		let read_error = unsafe { ffi::JDX_WriteDatasetToPath(dataset_ptr, path_cstring.as_ptr()) };

		if let Some(error) = Error::new_with_path(read_error, path) {
			return Err(error);
		}

		unsafe {
			ffi::JDX_FreeDataset(dataset_ptr);
		}

		return Ok(());
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
				header.image_count as usize,
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

pub struct ImageIterator<'a> {
	dataset: &'a Dataset,
	index: usize,
}

impl Iterator for ImageIterator<'_> {
	type Item = Image;

	fn next(&mut self) -> Option<Image> {
		if self.index >= self.len() {
			return None;
		}

		let image_size = self.dataset.header.image_size();
		let start_data = self.index * image_size;
		let end_data = start_data + image_size;

		let raw_data = self.dataset.image_data[start_data..end_data].to_vec();
		let label_index = self.dataset.label_data[self.index];

		return Some(Image {
			raw_data: raw_data,
			width: self.dataset.header.image_width,
			height: self.dataset.header.image_height,
			bit_depth: self.dataset.header.bit_depth,
			label: self.dataset.header.labels[label_index as usize].clone(),
			label_index: label_index,
		});
	}
}

impl ExactSizeIterator for ImageIterator<'_> {
	fn len(&self) -> usize {
		self.dataset.header.image_count as usize
	}
}
