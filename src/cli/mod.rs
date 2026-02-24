//! # Command-Line Interface (CLI) Module
//!
//! This module serves as the entry point for handling all command-line interactions
//! for the Raze archiving utility. It leverages the `clap` crate to provide robust
//! argument parsing, subcommand dispatching, and automatic generation of help messages.
//!
//! The module is structured to clearly separate argument definitions (`args.rs`)
//! from the specific command implementations (`commands.rs`), ensuring a clean
//! and maintainable interface for users.
pub mod args;
pub mod commands;
