# Rnamer

[![CI](https://github.com/SergioGasquez/rnamer/actions/workflows/ci.yaml/badge.svg)](https://github.com/SergioGasquez/rnamer/actions/workflows/ci.yaml)
![MSRV](https://img.shields.io/badge/MSRV-1.64-blue?labelColor=1C2C2E&logo=Rust)

Command-line application that renames a Rust project.


## Features

- Renames the Rust project by modifying the `name` field of the `Cargo.toml` file

## Usage

```console
Usage: rnamer [OPTIONS] --new-name <NEW_NAME>

Options:
  -n, --new-name <NEW_NAME>  GitHub username
  -p, --path <PATH>          Weeks to look back [default: Cargo.toml]
  -h, --help                 Print help
  -V, --version              Print version
```

## Installation

To install Rnamer, you must first have Rust installed. You can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can install Rnamer using Cargo:

```console
cargo install rnamer --git https://github.com/SergioGasquez/rnamer
```

This will download the source code, compile it, and install the binary in your system.

## Contributing

Contributions are welcome! If you encounter a bug or have a feature request, please open an issue on the [GitHub repository](https://github.com/SergioGasquez/rnamer). If you would like to contribute code, please fork the repository and submit a pull request.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
