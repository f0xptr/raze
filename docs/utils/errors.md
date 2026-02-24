# errors.rs Documentation

This document provides an overview of `src/utils/errors.rs`, which defines the custom error types used throughout the Raze application.

## Overview

This module is dedicated to defining the custom error types utilized across the Raze application. By centralizing these error definitions, it establishes a consistent and robust mechanism for reporting failures encountered during various operations, including I/O, compression, and decompression tasks.

The `thiserror` crate is integrated to facilitate the derivation of comprehensive error messages and streamline error handling and propagation within the application.

## Enums

### `enum RazeError`

Represents all possible errors that can occur within the Raze application.

This enum encapsulates a variety of failure modes, offering specific error variants tailored to different contexts. Each variant incorporates a descriptive error message generated via `#[error(...)]` attributes, ensuring user-friendly output.

#### Variants

*   `Io(#[from] io::Error)`

    Indicates an input/output operation failure.

    This error typically arises when file system operations, such as reading from or writing to files, fail due to underlying system issues.

*   `NotFound(String)`

    Indicates that the specified file or directory could not be found.

    This error occurs when a provided path does not exist on the file system or cannot be accessed.

*   `CompressionError(String)`

    Indicates an error specific to the Zstandard compression process.

    This error is returned when issues emerge during the compression of data into an `.rz` archive, for example, if the Zstandard encoder encounters an unexpected state.

*   `DecompressionError(String)`

    Indicates an error specific to the decompression process.

    This error occurs when problems are encountered during the extraction of data from an `.rz` archive, such as corrupted archive data or issues with the Zstandard decoder.
