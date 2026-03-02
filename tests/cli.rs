//! # Integration Tests for the Raze CLI
//!
//! This module contains integration tests for the Raze command-line interface.

use std::fs::{self, File};
use std::io::Write;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_cli_pack_unpack_flow() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_file.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "Hello, Raze CLI!").unwrap();

    let archive_path = dir.path().join("archive.rz");
    let unpack_dir = tempdir().unwrap();

    // Test packing
    let output = Command::new("./target/debug/raze")
        .arg("--pack")
        .arg("-s")
        .arg(dir.path())
        .arg("-o")
        .arg(&archive_path)
        .output()
        .unwrap();

    assert!(output.status.success(), "CLI pack command failed");

    // Test unpacking
    let output = Command::new("./target/debug/raze")
        .arg("--unpack")
        .arg("-a")
        .arg(&archive_path)
        .arg("-d")
        .arg(unpack_dir.path())
        .output()
        .unwrap();

    assert!(output.status.success(), "CLI unpack command failed");

    let unpacked_file_path = unpack_dir
        .path()
        .join(dir.path().file_name().unwrap())
        .join("test_file.txt");
    assert!(unpacked_file_path.exists());
    let content = fs::read_to_string(unpacked_file_path).unwrap();
    assert_eq!(content.trim(), "Hello, Raze CLI!");
}

#[test]
fn test_cli_encryption_flow() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_file.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "Hello, Encrypted Raze!").unwrap();

    let archive_path = dir.path().join("archive.rz");
    let unpack_dir = tempdir().unwrap();

    // Test packing with password
    let output = Command::new("./target/debug/raze")
        .arg("--pack")
        .arg("-s")
        .arg(&file_path)
        .arg("-o")
        .arg(&archive_path)
        .arg("-p")
        .arg("password123")
        .output()
        .unwrap();

    assert!(output.status.success(), "CLI pack with encryption failed");

    // Test unpacking with correct password
    let output = Command::new("./target/debug/raze")
        .arg("--unpack")
        .arg("-a")
        .arg(&archive_path)
        .arg("-d")
        .arg(unpack_dir.path())
        .arg("-p")
        .arg("password123")
        .output()
        .unwrap();

    assert!(output.status.success(), "CLI unpack with encryption failed");

    let unpacked_file_path = unpack_dir.path().join("test_file.txt");
    assert!(unpacked_file_path.exists());
    let content = fs::read_to_string(unpacked_file_path).unwrap();
    assert_eq!(content.trim(), "Hello, Encrypted Raze!");

    // Test unpacking with wrong password
    let unpack_dir_fail = tempdir().unwrap();
    let output = Command::new("./target/debug/raze")
        .arg("--unpack")
        .arg("-a")
        .arg(&archive_path)
        .arg("-d")
        .arg(unpack_dir_fail.path())
        .arg("-p")
        .arg("wrongpassword")
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "CLI unpack with wrong password should fail"
    );
}
