//! # Integration Tests for Compression and Decompression
//!
//! This module contains integration tests designed to verify the end-to-end
//! functionality of the Raze archiving utility's `pack` and `unpack` operations.
//! These tests ensure that files and directories can be correctly compressed into
//! `.rz` archives and subsequently extracted with their integrity preserved.
//!
//! All tests utilize `tempfile` to create isolated temporary directories and files,
//! guaranteeing that test runs do not interfere with the actual file system
//! and are clean upon completion.

use raze::core::{compress, decompress};
use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

/// Tests the complete workflow of packing a directory and then unpacking it.
///
/// This test performs the following steps:
/// 1. **Setup**: Creates a temporary directory containing a single text file
///    with predefined content.
/// 2. **Pack**: Invokes the `compress::pack` function to compress the entire
///    temporary directory into an `.rz` archive.
/// 3. **Unpack**: Calls `decompress::unpack` to extract the contents of the
///    newly created archive into another temporary destination directory.
/// 4. **Verify**: Asserts that the extracted file exists in the correct
///    location within the unpacked directory structure and that its content
///    exactly matches the original file's content.
/// 5. **Teardown**: The temporary directories are automatically cleaned up
///    when they go out of scope.
#[test]
fn test_pack_unpack_flow() {
    // 1. Setup: Create a temporary directory and a file with content
    let dir = tempdir().expect("Failed to create temporary directory");
    let file_path = dir.path().join("test_file.txt");
    let mut file = File::create(&file_path).expect("Failed to create test file");
    writeln!(file, "Hello, Raze!").expect("Failed to write to test file");

    // 2. Pack: Compress the directory
    let archive_path = dir.path().join("archive.rz");
    compress::pack(dir.path(), &archive_path).expect("Failed to pack directory");

    // 3. Unpack: Decompress the archive to a new directory
    let unpack_dir = tempdir().expect("Failed to create temporary unpack directory");
    decompress::unpack(&archive_path, unpack_dir.path()).expect("Failed to unpack archive");

    // 4. Verify: Check if the unpacked file exists and its content is correct
    // The unpacked directory structure preserves the original top-level directory name.
    let unpacked_file_path = unpack_dir
        .path()
        .join(
            dir.path()
                .file_name()
                .expect("Could not get temporary directory name"),
        )
        .join("test_file.txt");
    assert!(
        unpacked_file_path.exists(),
        "Unpacked file does not exist at {:?}",
        unpacked_file_path
    );
    let content = fs::read_to_string(unpacked_file_path).expect("Failed to read unpacked file");
    assert_eq!(content.trim(), "Hello, Raze!");

    // 5. Teardown: tempdir will automatically clean up the directories
}

/// Tests the complete workflow of packing a single file and then unpacking it.
///
/// This test focuses on the scenario where only a single file is compressed
/// rather than an entire directory. It performs the following steps:
/// 1. **Setup**: Creates a temporary directory and a single text file
///    with predefined content.
/// 2. **Pack**: Invokes the `compress::pack` function to compress the individual
///    file into an `.rz` archive.
/// 3. **Unpack**: Calls `decompress::unpack` to extract the contents of the
///    newly created archive into another temporary destination directory.
/// 4. **Verify**: Asserts that the extracted file exists directly within the
///    unpacked directory and that its content exactly matches the original file's content.
/// 5. **Teardown**: The temporary directories are automatically cleaned up
///    when they go out of scope.
#[test]
fn test_pack_unpack_single_file_flow() {
    // 1. Setup: Create a temporary directory and a file with content
    let dir = tempdir().expect("Failed to create temporary directory");
    let file_path = dir.path().join("test_file.txt");
    let mut file = File::create(&file_path).expect("Failed to create test file");
    writeln!(file, "Hello, Raze!").expect("Failed to write to test file");

    // 2. Pack: Compress the file
    let archive_path = dir.path().join("archive.rz");
    compress::pack(&file_path, &archive_path).expect("Failed to pack single file");

    // 3. Unpack: Decompress the archive to a new directory
    let unpack_dir = tempdir().expect("Failed to create temporary unpack directory");
    decompress::unpack(&archive_path, unpack_dir.path()).expect("Failed to unpack archive");

    // 4. Verify: Check if the unpacked file exists and its content is correct
    // When a single file is packed, it's extracted directly into the destination.
    let unpacked_file_path = unpack_dir.path().join("test_file.txt");
    assert!(
        unpacked_file_path.exists(),
        "Unpacked single file does not exist at {:?}",
        unpacked_file_path
    );
    let content = fs::read_to_string(unpacked_file_path).expect("Failed to read unpacked file");
    assert_eq!(content.trim(), "Hello, Raze!");

    // 5. Teardown: tempdir will automatically clean up the directories
}
