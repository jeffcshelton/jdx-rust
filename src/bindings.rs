use std::mem;
use crate::jdx;

pub type JDXLabel = i16;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JDXVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

#[repr(C)]
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum JDXError {
    None,

    OpenFile,
    CloseFile,
    ReadFile,
    WriteFile,
    CorruptFile,

    UnequalWidths,
    UnequalHeights,
    UnequalBitDepths,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JDXImage {
    pub data: *mut u8,

    pub width: u16,
    pub height: u16,
    pub bit_depth: u8,
}

impl From<&jdx::Image> for JDXImage {
    fn from(image: &jdx::Image) -> Self {
        let mut data = image.data.clone();

        let libjdx_image = JDXImage {
            data: data.as_mut_ptr(),
            width: image.width,
            height: image.height,
            bit_depth: image.bit_depth,
        };

        mem::forget(data);
        return libjdx_image;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JDXHeader {
    pub version: JDXVersion,

    pub image_width: u16,
    pub image_height: u16,
    pub bit_depth: u8,

    pub item_count: u64,
    pub compressed_size: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JDXDataset {
    pub header: JDXHeader,

    pub images: *mut JDXImage,
    pub labels: *mut JDXLabel,
}

impl From<&jdx::Dataset> for JDXDataset {
    fn from(dataset: &jdx::Dataset) -> Self {
        let mut images = dataset.images
            .iter()
            .map(|image| image.into())
            .collect::<Vec<JDXImage>>();
        
        let mut labels = dataset.labels
            .clone();

        let libjdx_dataset = JDXDataset {
            header: dataset.header,
            images: images.as_mut_ptr(),
            labels: labels.as_mut_ptr(),
        };

        mem::forget(images);
        mem::forget(labels);

        return libjdx_dataset;
    }
}

extern "C" {
    pub static JDX_VERSION: JDXVersion;

    pub fn JDX_ReadHeaderFromPath(dest: *mut JDXHeader, path: *const u8) -> JDXError;
    pub fn JDX_ReadDatasetFromPath(dest: *mut JDXDataset, path: *const u8) -> JDXError;

    pub fn JDX_WriteDatasetToPath(dataset: JDXDataset, path: *const u8) -> JDXError;
    pub fn JDX_FreeDataset(dataset: JDXDataset);
}
