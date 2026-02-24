//! # Error Handling Module
//!
//! This module defines the custom error types used throughout the Raze application.
//! By centralizing error definitions, it provides a consistent and robust mechanism
//! for reporting failures across various operations, including I/O, compression,
//! and decompression.
//!
//! The `thiserror` crate is utilized to derive comprehensive error messages and
//! facilitate easy error handling and propagation.

use std::io;
use thiserror::Error;

/// Represents all possible errors that can occur within the Raze application.
///
/// This enum encapsulates various failure modes, providing specific error
/// variants for different contexts. Each variant includes a descriptive
/// error message generated using `#[error(...)]` for user-friendly output.
#[derive(Error, Debug)]
pub enum RazeError {
    /// Indicates an input/output operation failure.
    ///
    /// This error is typically returned when file system operations, such
    /// as reading from or writing to files, fail due to underlying system issues.
    #[error("I/O error occurred: {0}")]
    Io(#[from] io::Error),

    /// Indicates that the specified file or directory could not be found.
    ///
    /// This error occurs when a provided path does not exist on the file system
    /// or cannot be accessed.
    #[error("The specified path was not found: {0}")]
    NotFound(String),

    /// Indicates an error specific to the Zstandard compression process.
    ///
    /// This error is returned when issues arise during the compression of data
    /// into an `.rz` archive, for example, if the Zstandard encoder encounters
    /// an unexpected state.
    #[error("Compression error: {0}")]
    CompressionError(String),

    /// Indicates an error specific to the decompression process.
    ///
    /// This error occurs when problems are encountered during the extraction
    /// of data from an `.rz` archive, such as corrupted archive data or
    /// issues with the Zstandard decoder.
    #[error("Decompression error: {0}")]
    DecompressionError(String),
}
