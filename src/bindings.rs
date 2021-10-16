use std::mem;
use crate::jdx;

pub type JDXLabel = u16;

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
pub struct JDXItem {
    pub data: *mut u8,

    pub width: u16,
    pub height: u16,
    pub bit_depth: u8,

    pub label: JDXLabel,
}

impl From<&jdx::Item> for JDXItem {
    fn from(item: &jdx::Item) -> Self {
        let mut image_data = item.data.clone();

        let libjdx_item = JDXItem {
            data: image_data.as_mut_ptr(),
            width: item.width,
            height: item.height,
            bit_depth: item.bit_depth,
            label: item.label,
        };

        mem::forget(image_data);
        return libjdx_item;
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
    pub items: *mut JDXItem,
}

impl From<&jdx::Dataset> for JDXDataset {
    fn from(dataset: &jdx::Dataset) -> Self {
        let mut items = dataset.items
            .iter()
            .map(|item| item.into())
            .collect::<Vec<JDXItem>>();
        
        let libjdx_dataset = JDXDataset {
            header: dataset.header,
            items: items.as_mut_ptr(),
        };

        mem::forget(items);
        return libjdx_dataset;
    }
}

extern "C" {
    pub static JDX_VERSION: JDXVersion;

    pub fn JDX_ReadHeaderFromPath(dest: *mut JDXHeader, path: *const i8) -> JDXError;
    pub fn JDX_ReadDatasetFromPath(dest: *mut JDXDataset, path: *const i8) -> JDXError;

    pub fn JDX_WriteDatasetToPath(dataset: JDXDataset, path: *const i8) -> JDXError;
    pub fn JDX_FreeDataset(dataset: JDXDataset);
}
