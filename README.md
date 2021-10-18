# JDX Rust

jdx-rust is a Rust wrapper library around libjdx, the low-level C library that manages JDX files directly. It can be used in any Rust project where you need to interact directly and conveniently with JDX files, and is actively used in the [JDX Command Line Tool](https://github.com/jeffreycshelton/jdx-clt).

## Usage

jdx-rust is not yet listed on [crates.io](https://crates.io), Rust's official crate registry (although it should be soon), so it must be listed as a dependency directly from GitHub in your Cargo.toml:

```toml
[dependencies]
jdx-rust = { git = "https://github.com/jeffreycshelton/jdx-rust" }
```

## Examples

***Coming soon***

## Development

jdx-rust and the rest of the [JDX Project](https://github.com/jeffreycshelton/jdx) are in early development and under constant change. New features and bug fixes will be added frequently, so check back here often! Also, if you enjoy using jdx-rust and would like to contribute to its development, please do! Contributions and issue submissions are welcome and help to make JDX a more capable format and tool.

## License

The JDX Rust wrapper is licensed under the [MIT License](LICENSE).