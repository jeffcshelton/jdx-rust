#[derive(Debug, Eq, PartialEq)]
pub struct Image<'a> {
	pub raw_data: &'a [u8],

	pub width: u16,
	pub height: u16,
	pub bit_depth: u8,

	pub label: &'a str,
	pub label_index: u16,
}
