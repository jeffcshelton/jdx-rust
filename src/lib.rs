mod bindings;

pub mod jdx {
    use std::{ptr, result, slice};
    use crate::bindings;

    pub type Label = i16;
    pub type Header = bindings::JDXHeader;
    pub type Version = bindings::JDXVersion;

    impl Header {
        pub fn read_from_path<S: Into<String>>(path: S) -> Result<Self> {
            let path_string = path.into();
            let mut header = Header { // Initialization done only to appease the borrow checker
                version: Version { major: 0, minor: 0, patch: 0 },
                image_width: 0,
                image_height: 0,
                bit_depth: 0,
                item_count: 0,
                compressed_size: 0,
            };

            let libjdx_error = unsafe {
                bindings::JDX_ReadHeaderFromPath(&mut header, path_string.as_ptr())
            };

            match libjdx_error {
                bindings::JDXError::None => Ok(header),
                bindings::JDXError::OpenFile => Err(Error::OpenFile(path_string)),
                bindings::JDXError::ReadFile => Err(Error::ReadFile(path_string)),
                bindings::JDXError::CorruptFile => Err(Error::CorruptFile(path_string)),
                bindings::JDXError::CloseFile => Err(Error::CloseFile(path_string)),
                _ => Err(Error::ReadFile(path_string))
            }
        }
    }

    impl Version {
        pub fn current() -> Self {
            unsafe { bindings::JDX_VERSION }
        }
    }
    
    #[derive(Clone)]
    pub struct Image {
        pub data: Vec<u8>,

        pub width: u16,
        pub height: u16,
        pub bit_depth: u8,
    }

    impl From<bindings::JDXImage> for Image {
        fn from(libjdx_image: bindings::JDXImage) -> Self {
            let image_size =
                libjdx_image.width as usize *
                libjdx_image.height as usize *
                (libjdx_image.bit_depth / 8) as usize;
            
            let image_data = unsafe {
                slice::from_raw_parts(libjdx_image.data, image_size).to_vec()
            };

            Image {
                data: image_data,
                width: libjdx_image.width,
                height: libjdx_image.height,
                bit_depth: libjdx_image.bit_depth,
            }
        }
    }

    #[derive(Clone)]
    pub struct Dataset {
        pub header: Header,

        pub images: Vec<Image>,
        pub labels: Vec<Label>,
    }

    impl From<bindings::JDXDataset> for Dataset {
        fn from(libjdx_dataset: bindings::JDXDataset) -> Self {
            let images = unsafe {
                slice::from_raw_parts(libjdx_dataset.images, libjdx_dataset.header.item_count as usize)
                    .iter()
                    .map(|libjdx_image| (*libjdx_image).into())
                    .collect()
            };

            let labels = unsafe {
                slice::from_raw_parts(libjdx_dataset.labels, libjdx_dataset.header.item_count as usize).to_vec()
            };

            Dataset {
                header: libjdx_dataset.header,
                images: images,
                labels: labels,
            }
        }
    }

    impl Dataset {
        pub fn read_from_path<S: Into<String>>(path: S) -> Result<Self> {
            let path_string = path.into();
            let mut libjdx_dataset = bindings::JDXDataset { // Initialization done only to appease the borrow checker
                header: Header {
                    version: Version { major: 0, minor: 0, patch: 0 },
                    image_width: 0,
                    image_height: 0,
                    bit_depth: 0,
                    item_count: 0,
                    compressed_size: 0,
                },
                images: ptr::null_mut(),
                labels: ptr::null_mut(),
            };

            let libjdx_error = unsafe {
                bindings::JDX_ReadDatasetFromPath(&mut libjdx_dataset, path_string.as_ptr())
            };

            let result = match libjdx_error {
                bindings::JDXError::None => Ok(libjdx_dataset.into()),
                bindings::JDXError::OpenFile => Err(Error::OpenFile(path_string)),
                bindings::JDXError::ReadFile => Err(Error::ReadFile(path_string)),
                bindings::JDXError::CorruptFile => Err(Error::CorruptFile(path_string)),
                bindings::JDXError::CloseFile => Err(Error::CloseFile(path_string)),
                _ => Err(Error::ReadFile(path_string))
            };

            unsafe {
                bindings::JDX_FreeDataset(libjdx_dataset);
            }

            return result;
        }

        pub fn write_to_path<S: Into<String>>(&self, path: S) -> Result<()> {
            let path_string = path.into();

            let libjdx_error = unsafe {
                bindings::JDX_WriteDatasetToPath(self.into(), path_string.as_ptr())
            };

            match libjdx_error {
                bindings::JDXError::None => Ok(()),
                bindings::JDXError::OpenFile => Err(Error::OpenFile(path_string)),
                bindings::JDXError::WriteFile => Err(Error::WriteFile(path_string)),
                bindings::JDXError::CloseFile => Err(Error::CloseFile(path_string)),
                _ => Err(Error::WriteFile(path_string))
            }
        }

        pub fn append(&mut self, dataset: &Dataset) -> Result<()> {
            if self.header.image_width != dataset.header.image_width {
                return Err(Error::UnequalWidths)
            } else if self.header.image_height != dataset.header.image_height {
                return Err(Error::UnequalHeights)
            } else if self.header.bit_depth != dataset.header.bit_depth {
                return Err(Error::UnequalBitDepths)
            }

            self.images.append(&mut dataset.images.clone());
            self.labels.append(&mut dataset.labels.clone());

            self.header.item_count += dataset.header.item_count;
            Ok(())
        }
    }

    #[derive(Clone)]
    pub enum Error {
        OpenFile(String),
        CloseFile(String),
        ReadFile(String),
        WriteFile(String),
        CorruptFile(String),

        UnequalWidths,
        UnequalHeights,
        UnequalBitDepths,
    }

    pub type Result<T> = result::Result<T, Error>;
}
