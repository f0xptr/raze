//! # Compression Module
//!
//! This module provides the core functionality for creating compressed `.rz` archives.
//! It leverages the `tar` crate for archiving multiple files or directories into a
//! single stream, and the `zstd` crate for highly efficient compression of that stream.
//!
//! The primary function, `pack`, orchestrates the entire compression process,
//! handling path validation, archive creation, and error management.

use crate::utils::errors::RazeError;
use crate::utils::security;
use log::info;
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::Path;
use tar::Builder;
use zstd::Encoder;

/// Compresses a given file or directory into a `.rz` archive using Zstandard.
/// Optionally encrypts the archive if a password is provided.
pub fn pack(
    source: impl AsRef<Path>,
    output: impl AsRef<Path>,
    password: Option<&str>,
) -> Result<(), RazeError> {
    let source_path = source.as_ref();
    let output_path = output.as_ref();

    if !source_path.exists() {
        return Err(RazeError::NotFound(source_path.display().to_string()));
    }

    info!(
        "Starting compression of '{}' into '{}'{}...",
        source_path.display(),
        output_path.display(),
        if password.is_some() {
            " with encryption"
        } else {
            ""
        }
    );

    // If we have a password, we'll compress into a buffer first, then encrypt that buffer into the final file.
    // For simplicity and to avoid large memory usage for very large files, we could use a temp file.
    // However, for "super complex security", we might want to avoid writing unencrypted data to disk.
    // Let's use a streaming approach.

    let final_output = File::create(output_path)?;

    if let Some(pwd) = password {
        // We need to stream Tar -> Zstd -> Encrypt -> File
        // We can use a pipe-like structure or just a simple intermediary buffer if we don't want to overcomplicate.
        // Actually, we can implement a custom Writer that encrypts on the fly,
        // but our encrypt_stream takes a Reader.

        // Let's use a temporary file for the compressed but unencrypted data.
        // This is a trade-off. To be even more secure, we'd do it all in memory or with a custom writer.
        let mut temp_file = tempfile::tempfile()?;

        {
            let encoder = Encoder::new(&temp_file, 3)
                .map_err(|e| RazeError::CompressionError(e.to_string()))?
                .auto_finish();

            let mut tar_builder = Builder::new(encoder);
            append_to_tar(&mut tar_builder, source_path)?;
            tar_builder.into_inner().map_err(|e| {
                RazeError::CompressionError(format!("Failed to finish archive: {}", e))
            })?;
        }

        // Now encrypt from temp_file to final_output
        temp_file.seek(SeekFrom::Start(0))?;
        security::encrypt_stream(temp_file, final_output, pwd)?;
    } else {
        let encoder = Encoder::new(final_output, 3)
            .map_err(|e| RazeError::CompressionError(e.to_string()))?
            .auto_finish();

        let mut tar_builder = Builder::new(encoder);
        append_to_tar(&mut tar_builder, source_path)?;
        tar_builder
            .into_inner()
            .map_err(|e| RazeError::CompressionError(format!("Failed to finish archive: {}", e)))?;
    }

    info!("Successfully created archive: {}", output_path.display());
    Ok(())
}

fn append_to_tar<W: std::io::Write>(
    tar_builder: &mut Builder<W>,
    source_path: &Path,
) -> Result<(), RazeError> {
    if source_path.is_dir() {
        tar_builder.append_dir_all(
            source_path
                .file_name()
                .unwrap_or_else(|| Path::new(".").as_os_str()),
            source_path,
        )?;
    } else {
        let mut file = File::open(source_path)?;
        tar_builder.append_file(
            source_path
                .file_name()
                .unwrap_or_else(|| Path::new(".").as_os_str()),
            &mut file,
        )?;
    }
    Ok(())
}
