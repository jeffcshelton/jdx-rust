use crate::*;
use std::fs;

#[test]
fn read_dataset() -> Result<()> {
	Dataset::read_from_path("./libjdx-sys/libjdx/res/example.jdx")
		.map(|_| ())
}

#[test]
fn write_dataset() -> Result<()> {
	let example_dataset = Dataset::read_from_path("./libjdx-sys/libjdx/res/example.jdx")?;
	example_dataset.write_to_path("./libjdx-sys/libjdx/res/temp.jdx")?;

	let read_dataset = Dataset::read_from_path("./libjdx-sys/libjdx/res/temp.jdx")?;
	assert_eq!(read_dataset, example_dataset);

	fs::remove_file("./libjdx-sys/libjdx/res/temp.jdx")
		.map_err(|_| Error::OpenFile("./libjdx-sys/libjdx/res/temp.jdx".to_owned()))?;

	return Ok(());
}

#[test]
fn read_header() -> Result<()> {
	let header = Header::read_from_path("./libjdx-sys/libjdx/res/example.jdx")?;

	assert_eq!(header.version, Version::current());
	assert_eq!(header.bit_depth, 24);
	assert_eq!(header.image_width, 52);
	assert_eq!(header.image_height, 52);
	assert_eq!(header.image_count, 8);

	return Ok(());
}
