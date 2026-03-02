# args.rs Documentation

This document provides an overview of `src/cli/args.rs`, which defines the top-level command-line argument structure for the Raze archiving utility.

## Overview

This module is responsible for defining the command-line argument structure for the Raze archiving utility. It leverages the `clap` crate to parse user input, offering a robust and user-friendly interface for interacting with Raze's features.

The central `RazeArgs` structure encapsulates all possible flags and options, guiding the application's execution flow based on the user's command-line invocation.

## Structs

### `struct RazeArgs`

`Raze`: A blazingly fast and lightweight archiving utility with secure encryption.

This structure represents the main entry point for parsing command-line arguments provided to the Raze CLI application. It uses `clap`'s `Parser` derive to automatically generate argument parsing logic, help messages, and version information.

#### Fields

*   `pack: bool`: A flag to activate packing mode.
*   `unpack: bool`: A flag to activate unpacking mode.
*   `source: Option<String>`: (Required for packing) The path to the source file or directory to be compressed.
*   `output: Option<String>`: (Required for packing) The name or path of the output .rz archive file.
*   `archive: Option<String>`: (Required for unpacking) The path to the .rz archive file to be decompressed.
*   `destination: Option<String>`: (Optional for unpacking) The destination directory for extraction. Defaults to the current directory.
*   `password: Option<String>`: (Optional) Password for encryption or decryption.
