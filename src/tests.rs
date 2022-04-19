use lazy_static::lazy_static;
use crate::*;
use std::fs;

const TEMP_PATH: &'static str = "./libjdx-sys/libjdx/res/temp.jdx";
const EXAMPLE_PATH: &'static str = "./libjdx-sys/libjdx/res/example.jdx";

lazy_static! {
	static ref EXAMPLE_DATASET: Dataset = {
		Dataset::read_from_path(EXAMPLE_PATH)
			.unwrap_or_else(|error| panic!("Cannot read example dataset: {}", error))
	};
}

#[test]
fn read_dataset() -> Result<()> {
	Dataset::read_from_path(EXAMPLE_PATH)
		.map(|_| ())
}

#[test]
fn write_dataset() -> Result<()> {
	EXAMPLE_DATASET.write_to_path(TEMP_PATH)?;

	let read_dataset = Dataset::read_from_path(TEMP_PATH)?;
	assert!(EXAMPLE_DATASET.eq(&read_dataset));

	fs::remove_file(TEMP_PATH)
		.map_err(|_| Error::OpenFile(TEMP_PATH.to_owned()))?;

	return Ok(());
}

#[test]
fn append_dataset() -> Result<()> {
	let mut copy = EXAMPLE_DATASET.clone();

	copy.append(&EXAMPLE_DATASET)?;

	assert_eq!(copy.header().image_count, EXAMPLE_DATASET.header().image_count * 2);
	assert_eq!(copy.get_image(0), copy.get_image(EXAMPLE_DATASET.header().image_count as usize));

	return Ok(());
}

#[test]
fn read_header() -> Result<()> {
	let header = Header::read_from_path(EXAMPLE_PATH)?;

	assert_eq!(header.version, Version::current());
	assert_eq!(header.bit_depth, 24);
	assert_eq!(header.image_width, 52);
	assert_eq!(header.image_height, 52);
	assert_eq!(header.image_count, 8);

	return Ok(());
}
