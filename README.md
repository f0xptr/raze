# Raze

Raze is a blazingly fast and lightweight archiving utility implemented in Rust. It provides a Command-Line Interface (CLI) for creating and extracting `.rz` archives. Raze combines the `tar` archiving format with the `Zstandard` compression algorithm to deliver efficient and high-performance file management.

## Features

-   **High Performance**: Utilizes Zstandard (Zstd) for superior compression and decompression speeds.
-   **Efficient Archiving**: Employs the Tar format for bundling multiple files and directories into a single stream.
-   **Simple CLI**: Offers straightforward `pack` and `unpack` commands.
-   **Robust Error Handling**: Custom error types ensure clear and descriptive feedback on operational failures.

## Installation

To get started with Raze, clone the repository and build it:

```bash
git clone https://github.com/f0xptr/raze.git
cd raze
cargo build --release
```

This will build the `raze` executable in the `target/release/` directory. You can then add this directory to your system's `PATH` or move the executable to a directory already in your `PATH` for easy access.

## Usage

### Packing (Compression)

To compress a file or directory:

```bash
raze pack -s <source_path> -o <output_archive.rz>
```

Example:
```bash
raze pack -s my_folder -o my_archive.rz
raze pack -s my_file.txt -o my_file.txt.rz
```

### Unpacking (Decompression)

To decompress an `.rz` archive:

```bash
raze unpack -a <archive.rz> -d <destination_directory>
```

Example:
```bash
raze unpack -a my_archive.rz -d extracted_files
```
If `-d` is not specified, it will unpack to the current directory.

## Contributing

We welcome contributions to Raze! Please see our [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to get started.

## License

This project is licensed under the [MIT License](LICENSE).
