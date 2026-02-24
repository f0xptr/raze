//! # CLI Arguments Module
//!
//! This module defines the top-level command-line argument structure for the Raze archiving utility.
//! It uses the `clap` crate to parse user input, providing a robust and user-friendly interface
//! for interacting with Raze's functionalities.
//!
//! The primary structure, `RazeArgs`, encapsulates all possible subcommands and global flags,
//! directing the application's flow based on the user's command-line invocation.

use super::commands::Commands;
use clap::Parser;

/// `Raze`: A blazingly fast and lightweight archiving utility.
///
/// This structure represents the main entry point for parsing command-line arguments
/// provided to the Raze CLI application. It utilizes `clap`'s `Parser` derive
/// to automatically generate argument parsing logic, help messages, and version information.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct RazeArgs {
    /// The subcommand to execute (e.g., `pack` for compression, `unpack` for decompression).
    ///
    /// This field determines the primary operation Raze will perform. Each subcommand
    /// (`Commands::Pack`, `Commands::Unpack`) is associated with its own set of arguments
    /// and specific behavior, as defined in the `commands` module.
    #[command(subcommand)]
    pub command: Commands,
}
