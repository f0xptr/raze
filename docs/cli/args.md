# args.rs Documentation

This document provides an overview of `src/cli/args.rs`, which defines the top-level command-line argument structure for the Raze archiving utility.

## Overview

This module is responsible for defining the command-line argument structure for the Raze archiving utility. It leverages the `clap` crate to parse user input, offering a robust and user-friendly interface for interacting with Raze's features.

The central `RazeArgs` structure encapsulates all possible subcommands and global flags, guiding the application's execution flow based on the user's command-line invocation.

## Structs

### `struct RazeArgs`

`Raze`: A blazingly fast and lightweight archiving utility.

This structure represents the main entry point for parsing command-line arguments provided to the Raze CLI application. It uses `clap`'s `Parser` derive to automatically generate argument parsing logic, help messages, and version information.

#### Fields

*   `command: Commands`

    The subcommand to execute (e.g., `pack` for compression, `unpack` for decompression).

    This field determines the primary operation Raze will perform. Each subcommand (`Commands::Pack`, `Commands::Unpack`) is associated with its own set of arguments and specific behavior, as defined in the `commands` module.
