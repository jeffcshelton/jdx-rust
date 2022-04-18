use std::{
	env,
	fs,
	io,
	path::{Path, PathBuf},
	process::Command,
};

#[derive(Debug)]
enum BuildError {
	EnvVarNotSet(String),
	IoFailure(io::ErrorKind),
	GitFailure,
	TargetUnsupported(String),
}

fn main() -> Result<(), BuildError> {
	let target = env::var("TARGET")
		.map_err(|_| BuildError::EnvVarNotSet("TARGET".to_owned()))?
		.to_lowercase();

	let profile = env::var("PROFILE")
		.map_err(|_| BuildError::EnvVarNotSet("PROFILE".to_owned()))?
		.to_lowercase();

	if target.contains("windows") {
		Err(BuildError::TargetUnsupported(target))?;
	}

	if !Path::new("./libjdx/libdeflate/lib").exists() {
		Command::new("git")
			.args(&["submodule", "update", "--init", "--recursive"])
			.status()
			.map_err(|_| BuildError::GitFailure)?;
	}

	let to_src = |dir_entry: fs::DirEntry| -> Option<PathBuf> {
		(
			dir_entry
				.file_type()
				.map_or(false, |file_type| file_type.is_file())

			&& dir_entry
				.path()
				.extension()
				.and_then(|ext| Some(ext == "c"))
				.unwrap_or(false)
		).then(|| dir_entry.path())
	};

	let libjdx_src = fs::read_dir("./libjdx/src")
		.map_err(|io_error| BuildError::IoFailure(io_error.kind()))?
		.map(Result::unwrap) // TODO: Consider removing
		.filter_map(to_src);

	let libdeflate_src = fs::read_dir("./libjdx/libdeflate/lib")
		.map_err(|io_error| BuildError::IoFailure(io_error.kind()))?
		.map(Result::unwrap)
		.filter_map(to_src);

	let mut build = cc::Build::new();

	build
		.extra_warnings(false)
		.includes(["./libjdx/include", "./libjdx/libdeflate"])
		.files(libjdx_src)
		.files(libdeflate_src)
		.static_flag(true);

	if profile == "debug" {
		build
			.define("DEBUG", None)
			.debug(true)
			.flag_if_supported("-fsanitize=address")
			.opt_level(0);
	} else if profile == "release" {
		build
			.define("RELEASE", None)
			.force_frame_pointer(false)
			.opt_level(3);
	}

	build.compile("jdx");
	return Ok(());
}
