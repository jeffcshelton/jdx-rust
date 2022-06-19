use lazy_static::lazy_static;
use crate::*;
use std::fs;

const TEMP_PATH: &'static str = "./res/temp.jdx";
const EXAMPLE_PATH: &'static str = "./res/example.jdx";

lazy_static! {
	static ref EXAMPLE_DATASET: Dataset = {
		Dataset::read_from_path(EXAMPLE_PATH)
			.unwrap_or_else(|error| panic!("Cannot read example dataset: {error}"))
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

	fs::remove_file(TEMP_PATH)?;
	Ok(())
}

#[test]
fn append_dataset() -> Result<()> {
	let mut copy1 = EXAMPLE_DATASET.clone();
	let copy2 = copy1.clone();
	copy1.append(copy2)?;

	assert_eq!(copy1.header().classes, EXAMPLE_DATASET.header().classes);
	assert_eq!(copy1.header().image_count, EXAMPLE_DATASET.header().image_count * 2);
	assert_eq!(copy1.get(0), copy1.get(EXAMPLE_DATASET.header().image_count as usize));

	Ok(())
}

#[test]
fn read_header() -> Result<()> {
	let header = Header::read_from_path(EXAMPLE_PATH)?;

	assert_eq!(header.version, Version::V0);
	assert_eq!(header.bit_depth, 24);
	assert_eq!(header.image_width, 52);
	assert_eq!(header.image_height, 52);
	assert_eq!(header.image_count, 8);

	Ok(())
}
