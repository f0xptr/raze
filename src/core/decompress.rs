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
use crate::utils::security;
use log::info;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use tar::Archive;
use zstd::Decoder;

/// Extracts a `.rz` archive into a specified destination directory.
/// Optionally decrypts the archive if a password is provided.
pub fn unpack(
    archive_path: impl AsRef<Path>,
    destination: impl AsRef<Path>,
    password: Option<&str>,
) -> Result<(), RazeError> {
    let archive_path = archive_path.as_ref();
    if !archive_path.exists() {
        return Err(RazeError::NotFound(archive_path.display().to_string()));
    }

    let destination_path = destination.as_ref();
    fs::create_dir_all(destination_path)?;

    let mut archive_file = File::open(archive_path)?;

    // Check if the file is encrypted by reading the magic header
    let mut magic = [0u8; 4];
    let is_encrypted = if archive_file.read_exact(&mut magic).is_ok() {
        magic == *b"RZCR"
    } else {
        false
    };
    archive_file.seek(SeekFrom::Start(0))?;

    if is_encrypted {
        let pwd = password.ok_or_else(|| {
            RazeError::CryptoError("Archive is encrypted but no password was provided".to_string())
        })?;

        // Decrypt to a temporary file
        let mut temp_file = tempfile::tempfile()?;
        security::decrypt_stream(&archive_file, &mut temp_file, pwd)?;

        temp_file.seek(SeekFrom::Start(0))?;
        let decoder =
            Decoder::new(temp_file).map_err(|e| RazeError::DecompressionError(e.to_string()))?;
        let mut tar_archive = Archive::new(decoder);

        info!(
            "Extracting encrypted archive '{}' to '{}'...",
            archive_path.display(),
            destination_path.display()
        );
        tar_archive.unpack(destination_path)?;
    } else {
        if password.is_some() {
            info!("Warning: Password provided but archive does not appear to be encrypted.");
        }
        let decoder =
            Decoder::new(archive_file).map_err(|e| RazeError::DecompressionError(e.to_string()))?;
        let mut tar_archive = Archive::new(decoder);

        info!(
            "Extracting '{}' to '{}'...",
            archive_path.display(),
            destination_path.display()
        );
        tar_archive.unpack(destination_path)?;
    }

    info!(
        "Successfully extracted archive to: {}",
        destination_path.display()
    );
    Ok(())
}
