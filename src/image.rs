#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Image {
	pub raw_data: Vec<u8>,

	pub width: u16,
	pub height: u16,
	pub bit_depth: u8,

	pub label: String,
	pub label_index: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Img<'a> {
	pub raw_data: &'a [u8],

	pub width: u16,
	pub height: u16,
	pub bit_depth: u8,

	pub label: &'a str,
	pub label_index: u16,
}

impl Img<'_> {
	pub fn to_owned(&self) -> Image {
		Image {
			raw_data: self.raw_data.to_vec(),
			width: self.width,
			height: self.height,
			bit_depth: self.bit_depth,
			label: self.label.to_owned(),
			label_index: self.label_index,
		}
	}
}
