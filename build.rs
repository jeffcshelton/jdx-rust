use std::{env, io, process::Command};

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

    println!("cargo:rustc-link-search=libjdx/lib");
    println!("cargo:rustc-link-lib=[static]jdx");

    Ok(())
}
