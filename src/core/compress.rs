//! # Compression Module
//!
//! This module provides the core functionality for creating compressed `.rz` archives.
//! It leverages the `tar` crate for archiving multiple files or directories into a
//! single stream, and the `zstd` crate for highly efficient compression of that stream.
//!
//! The primary function, `pack`, orchestrates the entire compression process,
//! handling path validation, archive creation, and error management.

use crate::utils::errors::RazeError;
use log::info;
use std::fs::File;
use std::path::Path;
use tar::Builder;
use zstd::Encoder;

/// Compresses a given file or directory into a `.rz` archive using Zstandard.
///
/// This function takes a source path (either a file or a directory) and an output
/// path for the resulting `.rz` archive. It uses a `tar` builder to create an
/// archive stream, which is then compressed using the Zstandard algorithm.
///
/// # Arguments
///
/// * `source` - A path-like object (`impl AsRef<Path>`) representing the file
///   or directory to be compressed. The contents of this path will be added
///   to the archive.
/// * `output` - A path-like object (`impl AsRef<Path>`) specifying the full path
///   (including the filename and `.rz` extension) where the compressed archive
///   will be saved.
///
/// # Errors
///
/// This function can return a `RazeError` in the following cases:
/// - `RazeError::NotFound`: If the `source` path does not exist.
/// - `RazeError::Io`: If any I/O operation (e.g., file creation, reading, writing) fails.
/// - `RazeError::CompressionError`: If an error occurs during the Zstandard compression
///   process or while finalizing the tar archive.
///
/// # Examples
///
/// ```no_run
/// use raze::core::compress;
/// use std::path::Path;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Compress a single file
///     compress::pack("my_document.txt", "my_document.txt.rz")?;
///
///     // Compress an entire directory
///     compress::pack("my_project_folder", "my_project_folder.rz")?;
///     Ok(())
/// }
/// ```
pub fn pack(source: impl AsRef<Path>, output: impl AsRef<Path>) -> Result<(), RazeError> {
    let source_path = source.as_ref();
    let output_path = output.as_ref();

    if !source_path.exists() {
        return Err(RazeError::NotFound(source_path.display().to_string()));
    }

    info!(
        "Starting compression of '{}' into '{}'...",
        source_path.display(),
        output_path.display()
    );

    let output_file = File::create(output_path)?;
    // Initialize the Zstandard encoder with a balanced compression level (3).
    let encoder = Encoder::new(output_file, 3)
        .map_err(|e| RazeError::CompressionError(e.to_string()))?
        .auto_finish(); // Automatically finish the Zstd stream on drop.

    // Wrap the Zstd encoder in a Tar builder to create the archive structure.
    let mut tar_builder = Builder::new(encoder);

    if source_path.is_dir() {
        // If the source is a directory, append all its contents recursively.
        // The `file_name()` is used to ensure the directory itself is the root
        // within the archive, rather than its parent path.
        tar_builder.append_dir_all(
            source_path
                .file_name()
                .unwrap_or_else(|| Path::new(".").as_os_str()),
            source_path,
        )?;
    } else {
        // If the source is a file, open it and append it to the archive.
        // The `file_name()` is used to determine the name of the file within the archive.
        let mut file = File::open(source_path)?;
        tar_builder.append_file(
            source_path
                .file_name()
                .unwrap_or_else(|| Path::new(".").as_os_str()),
            &mut file,
        )?;
    }

    // Ensure all buffered data is flushed and the archive is properly closed.
    tar_builder
        .into_inner()
        .map_err(|e| RazeError::CompressionError(format!("Failed to finish archive: {}", e)))?;

    info!("Successfully created archive: {}", output_path.display());
    Ok(())
}
