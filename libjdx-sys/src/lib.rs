use libc::c_char;

// TODO: Consider changing all types with pointers to have lifetimes to reduce copying

pub type JDXLabel = u16;

pub const JDX_BUILD_DEV: u8 = 0;
pub const JDX_BUILD_ALPHA: u8 = 1;
pub const JDX_BUILD_BETA: u8 = 2;
pub const JDX_BUILD_RC: u8 = 3;
pub const JDX_BUILD_RELEASE: u8 = 4;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JDXVersion {
	pub build_type: u8,
	pub patch: u8,
	pub minor: u8,
	pub major: u8
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum JDXError {
	None,

	OpenFile,
	CloseFile,
	ReadFile,
	WriteFile,
	CorruptFile,

	MemoryFailure,

	UnequalWidths,
	UnequalHeights,
	UnequalBitDepths,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JDXHeader {
	pub version: JDXVersion,

	pub image_count: u64,
	pub image_width: u16,
	pub image_height: u16,
	pub bit_depth: u8,

	pub labels: *mut *mut c_char,
	pub label_count: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JDXDataset {
	pub header: *mut JDXHeader,
	
	pub _raw_labels: *mut JDXLabel,
	pub _raw_image_data: *mut u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JDXImage {
	pub raw_data: *mut u8,

	pub width: u16,
	pub height: u16,
	pub bit_depth: u8,

	pub label_str: *mut c_char,
	pub label_num: JDXLabel,
}

#[link(name = "jdx", kind = "static")]
extern "C" {
	pub static JDX_VERSION: JDXVersion;

	pub fn JDX_CompareVersions(v1: JDXVersion, v2: JDXVersion) -> i32;

	pub fn JDX_AllocHeader() -> *mut JDXHeader;
	pub fn JDX_FreeHeader(header: *mut JDXHeader);
	pub fn JDX_CopyHeader(dest: *mut JDXHeader, src: *const JDXHeader);

	pub fn JDX_GetImageSize(header: *const JDXHeader) -> usize;

	pub fn JDX_ReadHeaderFromPath(dest: *mut JDXHeader, path: *const c_char) -> JDXError;

	pub fn JDX_AllocDataset() -> *mut JDXDataset;
	pub fn JDX_FreeDataset(dataset: *mut JDXDataset);
	pub fn JDX_CopyDataset(dest: *mut JDXDataset, src: *const JDXDataset);
	pub fn JDX_AppendDataset(dest: *mut JDXDataset, src: *const JDXDataset) -> JDXError;

	pub fn JDX_GetImage(dataset: *const JDXDataset, index: u64) -> *mut JDXImage;

	pub fn JDX_ReadDatasetFromPath(dest: *mut JDXDataset, path: *const c_char) -> JDXError;
	pub fn JDX_WriteDatasetToPath(dataset: *const JDXDataset, path: *const c_char) -> JDXError;

	pub fn JDX_FreeImage(image: *mut JDXImage);
}
