# lib.rs Documentation

This document provides an overview of `src/lib.rs`, which serves as the main library entry point for the Raze archiving utility.

## Overview

The `lib.rs` file defines the Raze Archiving Library, a fast and lightweight archiving utility implemented in Rust. This library delivers the core functionalities required for creating and extracting `.rz` archives, by combining the efficiency of the Zstandard compression algorithm with the versatility of the Tar archiving format.

The design philosophy emphasizes efficiency and simplicity, aiming to provide a robust solution for consolidating and compressing files and directories.

## Features

*   **High Performance**: Leverages Zstandard (Zstd) to achieve superior compression and decompression speeds.
*   **Efficient Archiving**: Uses the Tar format to bundle multiple files and directories into a single stream.
*   **Simple API**: Offers straightforward functions for `pack`ing (compressing) and `unpack`ing (decompressing) archives.
*   **Error Handling**: Custom error types are implemented to provide clear and descriptive feedback for operational failures.

## Structure

The library is organized into the following key modules:

*   `core`: Contains the fundamental logic for compression and decompression operations.
    *   `core::compress`: Implements the `pack` function, used for creating `.rz` archives.
    *   `core::decompress`: Implements the `unpack` function, used for extracting `.rz` archives.
*   `utils`: Provides essential utility functions, error definitions, and logging setup.
    *   `utils::errors`: Defines the custom error types (`RazeError`) specific to the library.
    *   `utils::logger`: Manages the initialization and configuration of the logging environment.

## Usage

To integrate Raze as a library into your project, add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
raze = "1.0.0" # Or the latest version
```

Subsequently, you can import and utilize its functions within your Rust code:

```no_run
use raze::core::{compress, decompress};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example: Compress a file
    compress::pack("source_file.txt", "archive.rz")?;

    // Example: Decompress an archive
    decompress::unpack("archive.rz", "destination_directory")?;
    Ok(())
}
```
