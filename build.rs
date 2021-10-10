use std::{
    env,
    io,
    path::Path,
    process::Command
};

fn main() -> Result<(), io::ErrorKind> {
    env::set_current_dir("./libjdx")
        .map_err(|_| io::ErrorKind::NotFound)?;

    if cfg!(unix) {
        Command::new("make")
            .arg("libjdx")
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

    Ok(())
}
