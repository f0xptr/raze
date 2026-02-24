//! # Raze Archiving Library
//!
//! Raze is a blazingly fast and lightweight archiving utility implemented in Rust.
//! This library provides the core functionalities for creating and extracting
//! `.rz` archives, leveraging the power of the Zstandard compression algorithm
//! and the widely-used Tar archiving format.
//!
//! The design prioritizes efficiency and simplicity, offering a robust solution
//! for consolidating and compressing files and directories.
//!
//! ## Features
//!
//! - **High Performance**: Utilizes Zstandard (Zstd) for superior compression
//!   and decompression speeds.
//! - **Efficient Archiving**: Employs the Tar format for bundling multiple
//!   files and directories into a single stream.
//! - **Simple API**: Provides straightforward functions for `pack`ing (compressing)
//!   and `unpack`ing (decompressing) archives.
//! - **Error Handling**: Custom error types ensure clear and descriptive feedback
//!   on operational failures.
//!
//! ## Structure
//!
//! The library is organized into the following key modules:
//!
//! - `core`: Contains the fundamental logic for compression and decompression.
//!   - `core::compress`: Implements the `pack` function for creating `.rz` archives.
//!   - `core::decompress`: Implements the `unpack` function for extracting `.rz` archives.
//! - `utils`: Provides utility functions, error definitions, and logging setup.
//!   - `utils::errors`: Defines custom error types (`RazeError`) for the library.
//!   - `utils::logger`: Handles the initialization and configuration of the logging environment.
//!
//! ## Usage
//!
//! To use Raze as a library, add it as a dependency in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! raze = "1.0.0" # Or the latest version
//! ```
//!
//! Then, you can import and use its functions:
//!
//! ```no_run
//! use raze::core::{compress, decompress};
//! use std::path::Path;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Example: Compress a file
//!     compress::pack("source_file.txt", "archive.rz")?;
//!
//!     // Example: Decompress an archive
//!     decompress::unpack("archive.rz", "destination_directory")?;
//!     Ok(())
//! }
//! ```
pub mod core;
pub mod utils;
