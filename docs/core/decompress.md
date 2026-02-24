# decompress.rs Documentation

This document provides an overview of `src/core/decompress.rs`, which handles the decompression and extraction of `.rz` archive files.

## Overview

This module is dedicated to the robust decompression and extraction of `.rz` archive files. It leverages the `zstd` library for efficient Zstandard decompression and `tar` for handling the archive structure, ensuring that files and directories are restored correctly to their specified destination.

## Functions

### `fn unpack(archive_path: PathBuf, destination: PathBuf) -> Result<(), RazeError>`

Decompresses a `.rz` archive and extracts its contents to a specified destination.

This function orchestrates the entire unpacking process. It first opens the `.rz` archive, then decompresses it using Zstandard, and finally extracts the tar archive to the target `destination` directory. The function ensures that the destination directory exists and handles potential I/O or decompression errors gracefully.

**Arguments:**

*   `archive_path`: A `PathBuf` representing the path to the `.rz` archive file that needs to be unpacked.
*   `destination`: A `PathBuf` specifying the directory where the contents of the archive will be extracted. This directory will be created if it does not already exist.

**Returns:**

Returns `Ok(())` if the archive is successfully decompressed and its contents are extracted, or a `RazeError` if an error occurs during file operations, decompression, or tar extraction.
