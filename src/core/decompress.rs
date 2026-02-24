//! # Decompression Module
//!
//! This module provides the core functionality for extracting contents from
//! compressed `.rz` archives. It utilizes the `zstd` crate to decompress
//! the archived data and the `tar` crate to extract the individual files
//! and directories from the resulting stream.
//!
//! The primary function, `unpack`, manages the entire decompression process,
//! including archive validation, directory creation, and error handling.

use crate::utils::errors::RazeError;
use log::info;
use std::fs;
use std::fs::File;
use std::path::Path;
use tar::Archive;
use zstd::Decoder;

/// Extracts a `.rz` archive into a specified destination directory.
///
/// This function takes the path to an existing `.rz` archive and a target
/// destination directory. It first decompresses the archive using Zstandard
/// and then uses `tar` to extract all contents to the specified location.
///
/// If the destination directory does not exist, it will be created.
///
/// # Arguments
///
/// * `archive_path` - A path-like object (`impl AsRef<Path>`) representing
///   the `.rz` archive file to be decompressed.
/// * `destination` - A path-like object (`impl AsRef<Path>`) specifying
///   the directory where the archive's contents will be extracted.
///
/// # Errors
///
/// This function can return a `RazeError` in the following cases:
/// - `RazeError::NotFound`: If the `archive_path` does not exist.
/// - `RazeError::Io`: If any I/O operation (e.g., file opening, directory creation,
///   writing extracted files) fails.
/// - `RazeError::DecompressionError`: If an error occurs during the Zstandard
///   decompression process.
///
/// # Examples
///
/// ```no_run
/// use raze::core::decompress;
/// use std::path::Path;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Decompress an archive into a specific directory
///     decompress::unpack("my_archive.rz", "extracted_files")?;
///
///     // Decompress an archive into the current directory
///     decompress::unpack("another_archive.rz", ".")?;
///     Ok(())
/// }
/// ```
pub fn unpack(
    archive_path: impl AsRef<Path>,
    destination: impl AsRef<Path>,
) -> Result<(), RazeError> {
    let archive_path = archive_path.as_ref();
    if !archive_path.exists() {
        return Err(RazeError::NotFound(archive_path.display().to_string()));
    }

    let destination_path = destination.as_ref();
    // Ensure the destination directory exists; create it if not.
    fs::create_dir_all(destination_path)?;

    let archive_file = File::open(archive_path)?;
    // Initialize the Zstandard decoder to read the compressed stream.
    let decoder =
        Decoder::new(archive_file).map_err(|e| RazeError::DecompressionError(e.to_string()))?;

    // Wrap the decoder in a Tar archive parser to extract contents.
    let mut tar_archive = Archive::new(decoder);

    info!(
        "Extracting '{}' to '{}'...",
        archive_path.display(),
        destination_path.display()
    );

    // Unpack all contents into the specified destination directory.
    tar_archive.unpack(destination_path)?;

    info!(
        "Successfully extracted archive to: {}",
        destination_path.display()
    );
    Ok(())
}
