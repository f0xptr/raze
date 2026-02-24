//! # Raze Command-Line Interface (CLI) Application
//!
//! This is the main binary entry point for the Raze archiving tool. Raze provides
//! a blazingly fast and lightweight CLI experience for creating and extracting
//! `.rz` archives. It combines the `tar` archiving format with the `Zstandard`
//! compression algorithm to deliver efficient and high-performance file management.
//!
//! The CLI parses user commands and dispatches them to the underlying Raze library
//! (`raze::core`) for execution. Global error handling ensures a graceful exit
//! for any operational failures.

mod cli; // Declares the `cli` module, containing argument parsing logic.

use self::cli::args::RazeArgs;
use self::cli::commands::Commands;
use clap::Parser;
use log::error;
use raze::core::{compress, decompress};
use raze::utils::{errors::RazeError, logger};
use std::path::PathBuf;

/// The main entry point for the Raze CLI application.
///
/// This function initializes the logging system, parses command-line arguments
/// using `clap`, and then delegates the execution to the `run` function.
/// It catches any `RazeError` returned by `run`, logs it, and exits the
/// application with a non-zero status code to indicate failure.
///
/// # Panics
///
/// This function does not explicitly panic, but unhandled errors from internal
/// operations would lead to a program termination, which are typically caught
/// by the `run` function's error handling.
fn main() {
    // Initialize the application's logging environment.
    // This allows for console output of info, warn, and error messages.
    logger::init();

    // Parse command-line arguments provided by the user and execute the main logic.
    if let Err(e) = run(RazeArgs::parse()) {
        // If an operation fails, log the error message and exit with an error code.
        error!("Operation failed: {}", e);
        std::process::exit(1);
    }
}

/// Executes the main application logic based on the parsed command-line arguments.
///
/// This function acts as the central dispatcher for Raze's operations. It matches
/// the provided subcommand (e.g., `Pack` or `Unpack`) and calls the corresponding
/// function from the `raze::core` library to perform the archiving task.
///
/// # Arguments
///
/// * `args` - A `RazeArgs` struct containing the parsed command and its associated
///   arguments from the command line.
///
/// # Returns
///
/// Returns `Ok(())` if the command executes successfully, or a `RazeError`
/// if any part of the archiving or compression/decompression process fails.
fn run(args: RazeArgs) -> Result<(), RazeError> {
    match args.command {
        Commands::Pack { source, output } => {
            // Handle the `pack` subcommand: compress a source into an archive.
            let mut output_path = PathBuf::from(output);
            // Ensure the output file has the `.rz` extension for consistency.
            if output_path.extension().is_none() || output_path.extension().unwrap() != "rz" {
                output_path.set_extension("rz");
            }
            compress::pack(source, output_path)
        },
        Commands::Unpack {
            archive,
            destination,
        } => {
            // Handle the `unpack` subcommand: decompress an archive to a destination.
            decompress::unpack(archive, destination)
        },
    }
}
