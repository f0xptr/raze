# main.rs Documentation

This document provides an overview of `src/main.rs`, the main entry point for the Raze Command-Line Interface (CLI) application.

## Overview

The `main.rs` file serves as the primary binary entry point for the Raze archiving tool. Raze offers a fast and lightweight CLI experience for creating and extracting `.rz` archives. It combines the `tar` archiving format with the `Zstandard` compression algorithm, ensuring efficient and high-performance file management.

The CLI is responsible for parsing user commands and delegating them to the underlying Raze library (`raze::core`) for execution. Global error handling is implemented to ensure a graceful exit in case of any operational failures.

## Functions

### `fn main()`

The main entry point for the Raze CLI application.

This function initializes the logging system, parses command-line arguments using `clap`, and then delegates the execution to the `run` function. It captures any `RazeError` returned by `run`, logs it, and exits the application with a non-zero status code to signal failure.

**Panics:**
This function does not explicitly panic. Unhandled errors from internal operations, which are typically caught by the `run` function's error handling, would otherwise lead to program termination.

### `fn run(args: RazeArgs) -> Result<(), RazeError>`

Executes the main application logic based on the parsed command-line arguments.

This function acts as the central dispatcher for Raze's operations. It matches the provided subcommand (e.g., `Pack` or `Unpack`) and invokes the corresponding function from the `raze::core` library to perform the archiving task.

**Arguments:**
* `args`: A `RazeArgs` struct containing the parsed command and its associated arguments from the command line.

**Returns:**
Returns `Ok(())` if the command executes successfully, or a `RazeError` if any part of the archiving, compression, or decompression process encounters a failure.
