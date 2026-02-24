# compress.rs Documentation

This document provides an overview of `src/core/compress.rs`, which provides the core functionality for creating compressed `.rz` archives.

## Overview

This module is responsible for the core functionality of creating compressed `.rz` archives. It utilizes the `tar` crate to archive multiple files or directories into a single stream, and the `zstd` crate for highly efficient compression of that stream using the Zstandard algorithm.

The primary function, `pack`, orchestrates the entire compression process. This includes handling path validation, managing archive creation, and robust error management throughout the operation.

## Functions

### `fn pack(source: impl AsRef<Path>, output: impl AsRef<Path>) -> Result<(), RazeError>`

Compresses a given file or directory into a `.rz` archive using Zstandard.

This function accepts a source path (which can be either a file or a directory) and an output path for the resulting `.rz` archive. It constructs an archive stream using a `tar` builder, which is then compressed by the Zstandard algorithm.

**Arguments:**

*   `source`: A path-like object (`impl AsRef<Path>`) representing the file or directory whose contents are to be compressed and added to the archive.
*   `output`: A path-like object (`impl AsRef<Path>`) specifying the complete path, including filename and the `.rz` extension, where the compressed archive will be stored.

**Errors:**

This function is designed to return a `RazeError` in the following scenarios:

*   `RazeError::NotFound`: If the specified `source` path does not exist on the file system.
*   `RazeError::Io`: If any input/output operation, such as file creation, reading, or writing, encounters a failure.
*   `RazeError::CompressionError`: If an error occurs during the Zstandard compression process or while the tar archive is being finalized.

**Examples:**

```no_run
use raze::core::compress;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compress a single file
    compress::pack("my_document.txt", "my_document.txt.rz")?;

    // Compress an entire directory
    compress::pack("my_project_folder", "my_project_folder.rz")?;
    Ok(())
}
```
