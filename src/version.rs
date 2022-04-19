use crate::ffi;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Version {
	pub major: u8,
	pub minor: u8,
	pub patch: u8,
	pub build_type: BuildType,
}

impl Version {
	#[inline]
	pub fn current() -> Self {
		unsafe { ffi::JDX_VERSION }.into()
	}
}

impl ToString for Version {
	fn to_string(&self) -> String {
		format!("v{}.{}.{}{}", self.major, self.minor, self.patch, self.build_type.postfix())
	}
}

impl From<ffi::JDXVersion> for Version {
	fn from(version: ffi::JDXVersion) -> Self {
		Self {
			major: version.major,
			minor: version.minor,
			patch: version.patch,
			build_type: version.build_type.into(),
		}
	}
}

impl From<Version> for ffi::JDXVersion {
	fn from(version: Version) -> Self {
		Self {
			major: version.major,
			minor: version.minor,
			patch: version.patch,
			build_type: version.build_type.into_int(),
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuildType {
	Dev,
	Alpha,
	Beta,
	Rc,
	Release,
	CorruptBuild,
}

impl BuildType {
	pub fn postfix(&self) -> String {
		match self {
			Self::Dev => " (dev build)",
			Self::Alpha => "-alpha",
			Self::Beta => "-beta",
			Self::Rc => "-rc",
			Self::Release => "",
			Self::CorruptBuild => " (corrupt build)",
		}.to_owned()
	}

	pub fn into_int(&self) -> u8 {
		match self {
			Self::Dev => ffi::JDX_BUILD_DEV,
			Self::Alpha => ffi::JDX_BUILD_ALPHA,
			Self::Beta => ffi::JDX_BUILD_BETA,
			Self::Rc => ffi::JDX_BUILD_RC,
			Self::Release => ffi::JDX_BUILD_RELEASE,
			Self::CorruptBuild => 255,
		}
	}
}

impl From<u8> for BuildType {
	fn from(build_type: u8) -> Self {
		match build_type {
			ffi::JDX_BUILD_DEV => Self::Dev,
			ffi::JDX_BUILD_ALPHA => Self::Alpha,
			ffi::JDX_BUILD_BETA => Self::Beta,
			ffi::JDX_BUILD_RC => Self::Rc,
			ffi::JDX_BUILD_RELEASE => Self::Release,
			_ => Self::CorruptBuild
		}
	}
}
