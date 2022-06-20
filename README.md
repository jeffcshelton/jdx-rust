# JDX Rust

jdx-rust is a Rust wrapper library around libjdx, the low-level C library that manages JDX files directly. It can be used in any Rust project where you need to interact directly and conveniently with JDX files, and is actively used in the [JDX Command Line Tool](https://github.com/jeffreycshelton/jdx-clt).

## Usage

jdx-rust is not yet listed on [crates.io](https://crates.io), Rust's official crate registry (although it should be soon), so it must be listed as a dependency directly from GitHub in your Cargo.toml:

```toml
[dependencies]
jdx-rust = { git = "https://github.com/jeffreycshelton/jdx-rust", tag="v0.4.0" }
```

## Examples

To read and iterate through a dataset from a JDX file:

```rust
use jdx::Dataset;

fn main() {
	let dataset = Dataset::read_from_path("path/to/file.jdx")
		.unwrap_or_else(|error| {
			panic!("Something went wrong!");
		});

	let header = dataset.get_header();

	// Get width, height, and bit depth of the images
	// NOTE: These values are guaranteed to be the same across all images in the dataset
	let width = header.image_width;
	let height = header.image_height;
	let bit_depth = header.bit_depth;

	// Each possible value of 'header.bit_depth' corresponds to the number of bits in each pixel:
	// - 8 bits => 1 bytes per pixel (luma)
	// - 24 bits => 3 bytes per pixel (red, green, blue)
	// - 32 bits => 4 bytes per pixel (red, green, blue, alpha)

	// Dataset::iter forms an iterator of tuples of all images and their labels
	for (image, label) in dataset.iter() {
		// Each image is a Box<[u8]>, which is its raw pixel data
		// Pixels can be accessed using normal methods of indexing an array, such as:
		let px = image.get(/* insert pixel index */);
		
		// Labels are u16 values which correspond to a class in the 'classes' field of a Header
		// The label can be used as an index to find the class name as a string:
		let class_name = header.classes[usize::from(label)];
	}

	// Additionally, a dataset can be indexed using the Dataset::get method:
	let (third_image, third_label) = dataset.get(2).unwrap();

	Ok(())
}
```

To write a dataset to a JDX file:

```rust
let dataset: Dataset = ...;

dataset.write_to_path("path/to/new/file.jdx");
```

To read only the header of a JDX file:

```rust
use jdx::Header;

fn main() {
	let header = Header::read_from_path("path/to/file.jdx")
		.unwrap_or_else(|error| {
			panic!("Something went wrong!");
		});
	
	// From the header, you can access:
	// 1) JDX specification version of the file (header.version)
	// 2) Width and height of the images in the dataset (header.image_width, header.image_height)
	// 3) Bit depth of the images (header.bit_depth)
	// 4) Number of images in the dataset (header.image_count)
	// 5) Classes (categories) that the images can be in (header.classes)
}
```

## Development

jdx-rust and the rest of the [JDX Project](https://github.com/jeffreycshelton/jdx) are in early development and under constant change. New features and bug fixes will be added frequently, so check back here often! Also, if you enjoy using jdx-rust and would like to contribute to its development, please do! Contributions and issue submissions are welcome and help to make JDX a more capable format and tool.

## License

The JDX Rust wrapper is licensed under the [MIT License](LICENSE).