#[repr(C)]
#[derive(Clone, Copy)]
pub struct JDXVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum JDXColorType {
    Gray = 1,
    RGB = 3,
    RGBA = 4
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JDXImage {
    pub data: *mut u8,

    pub width: i16,
    pub height: i16,
    pub color_type: JDXColorType,
}

pub type JDXLabel = i16;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JDXHeader {
    pub version: JDXVersion,
    pub color_type: JDXColorType,

    pub image_width: i16,
    pub image_height: i16,
    pub item_count: i64,
    pub compressed_size: i64,

    pub error: *const u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JDXDataset {
    pub header: JDXHeader,

    pub images: *mut JDXImage,
    pub labels: *mut JDXLabel,

    pub error: *const u8,
}

extern "C" {
    pub static JDX_Version: JDXVersion;

    pub fn JDX_ReadHeaderFromPath(path: *const u8) -> JDXHeader;
    pub fn JDX_ReadDatasetFromPath(path: *const u8) -> JDXDataset;

    pub fn JDX_WriteDatasetToPath(dataset: JDXDataset, path: *const u8);
    pub fn JDX_FreeDataset(dataset: JDXDataset);
}
