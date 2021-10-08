mod bindings;

pub mod jdx {
    use std::{io, ptr};
    use crate::bindings;

    pub type Version = bindings::JDXVersion;
    pub type ColorType = bindings::JDXColorType;
    
    #[derive(Clone)]
    pub struct Image {
        pub data: Box<[u8]>,

        pub width: i16,
        pub height: i16,
        pub color_type: ColorType,
    }

    pub type Label = i16;

    #[derive(Clone, Copy)]
    pub struct Header {
        pub version: Version,
        pub color_type: ColorType,

        pub image_width: i16,
        pub image_height: i16,
        pub item_count: usize,
    }

    #[derive(Clone)]
    pub struct Dataset {
        pub header: Header,

        pub images: Vec<Image>,
        pub labels: Vec<Label>,
    }

    impl Image {
        pub(super) fn from_c(c_image: bindings::JDXImage) -> Image {
            let image_size = c_image.width as usize * c_image.height as usize * c_image.color_type as usize;
            let boxed_data = unsafe { Box::from_raw(ptr::slice_from_raw_parts_mut(c_image.data, image_size)) };

            Image {
                data: boxed_data,
                width: c_image.width,
                height: c_image.height,
                color_type: c_image.color_type
            }
        }

        pub(super) fn to_c(&mut self) -> bindings::JDXImage {
            bindings::JDXImage {
                data: self.data.as_mut_ptr(),
                width: self.width,
                height: self.height,
                color_type: self.color_type
            }
        }
    }

    impl Header {
        pub(super) fn from_c(c_header: bindings::JDXHeader) -> Result<Header, String> {
            if !c_header.error.is_null() {
                return unsafe { 
                    Err(c_header.error.as_ref().unwrap().to_string())
                };
            }

            Ok(Header {
                version: c_header.version,
                color_type: c_header.color_type,
                image_width: c_header.image_width,
                image_height: c_header.image_height,
                item_count: c_header.item_count as usize
            })
        }

        pub(super) fn to_c(&self) -> bindings::JDXHeader {
            bindings::JDXHeader {
                version: self.version,
                color_type: self.color_type,
                image_width: self.image_width,
                image_height: self.image_height,
                item_count: self.item_count as i64,
                compressed_size: -1,
                error: ptr::null()
            }
        }

        pub fn from_file(path: &str) -> io::Result<Header> {
            let c_header = unsafe { bindings::JDX_ReadHeaderFromPath(path.as_ptr()) };
            let rust_header = Header::from_c(c_header)
                .map_err(|_| io::Error::last_os_error())?;

            Ok(rust_header)
        }
    }

    impl Dataset {
        fn from_c(c_dataset: bindings::JDXDataset) -> Result<Dataset, String> {
            if !c_dataset.error.is_null() {
                return unsafe {
                    Err(c_dataset.error.as_ref().unwrap().to_string())
                };
            }

            let c_images = unsafe { Box::from_raw(ptr::slice_from_raw_parts_mut(c_dataset.images, c_dataset.header.item_count as usize)) };
            let c_labels = unsafe { Box::from_raw(ptr::slice_from_raw_parts_mut(c_dataset.labels, c_dataset.header.item_count as usize)) };

            let r_header = Header::from_c(c_dataset.header)?;
            let mut r_images = Vec::with_capacity(c_images.len());
            let mut r_labels = Vec::with_capacity(c_labels.len());

            for i in 0..r_header.item_count {
                r_images.push(Image::from_c(c_images[i]));
                r_labels.push(c_labels[i] as Label);
            }

            Ok(Dataset {
                header: r_header,
                images: r_images,
                labels: r_labels
            })
        }

        fn to_c(&mut self) -> bindings::JDXDataset {
            let mut c_images: Vec<bindings::JDXImage> = self.images
                .iter_mut()
                .map(|image| image.to_c())
                .collect();

            bindings::JDXDataset {
                header: self.header.to_c(),
                images: c_images.as_mut_ptr(),
                labels: self.labels.as_mut_ptr(),
                error: ptr::null()
            }
        }

        pub fn read_from_file(path: &str) -> io::Result<Dataset> {
            let c_dataset = unsafe { bindings::JDX_ReadDatasetFromPath(path.as_ptr()) };
            let rust_dataset = Dataset::from_c(c_dataset)
                .map_err(|_| io::Error::last_os_error())?;

            unsafe { bindings::JDX_FreeDataset(c_dataset) };

            Ok(rust_dataset)
        }

        pub fn write_to_file(&mut self, path: &str) -> io::Result<()> {
            unsafe {
                bindings::JDX_WriteDatasetToPath(self.to_c(), path.as_ptr());
            }

            Ok(())
        }
    }
}
