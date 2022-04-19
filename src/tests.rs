use crate::*;
use std::fs;

#[test]
fn read_dataset() -> Result<()> {
	let dataset = Dataset::read_from_path("./libjdx-sys/libjdx/res/example.jdx")?;

	assert_eq!(dataset.header.version, Version::current());
	assert_eq!(dataset.header.image_count, 8);

	return Ok(());
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
