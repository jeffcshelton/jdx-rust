use std::{env, io, path::Path, process::{self, Command}};

fn main() -> Result<(), io::ErrorKind> {
	env::set_current_dir("./libjdx")
		.map_err(|_| io::ErrorKind::NotFound)?;
	
	let profile = env::var("PROFILE").unwrap();
	
	let libjdx_name: &str;
	let target: &str;

	match profile.as_str() {
		"release" => {
			libjdx_name = "jdx";
			target = "libjdx";
		},
		"debug" => {
			libjdx_name = "jdx_debug";
			target = "debug";
		},
		_ => process::exit(1),
	}

	if cfg!(unix) {
		Command::new("make")
			.arg(target)
			.status()
			.map_err(|_| io::ErrorKind::NotFound)?;
	} else if cfg!(windows) {
		return Err(io::ErrorKind::Unsupported);
	} else {
		return Err(io::ErrorKind::Unsupported);
	}

	let cargo_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let libjdx_link_dir = Path::new(&cargo_dir)
		.join("libjdx")
		.join("lib");

	println!("cargo:rustc-link-search={}", libjdx_link_dir.display());
	println!("cargo:rustc-link-lib=static={}", libjdx_name);

	Ok(())
}
