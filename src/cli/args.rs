//! # CLI Arguments Module
//!
//! This module defines the top-level command-line argument structure for the Raze archiving utility.
//! It uses the `clap` crate to parse user input, providing a robust and user-friendly interface
//! for interacting with Raze's functionalities.
//!
//! The primary structure, `RazeArgs`, encapsulates all possible subcommands and global flags,
//! directing the application's flow based on the user's command-line invocation.

use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about = "Raze: A blazingly fast and lightweight archiving utility with secure encryption.", long_about = None)]
#[command(propagate_version = true)]
#[command(group(ArgGroup::new("mode").required(true).args(&["pack", "unpack"])))]
pub struct RazeArgs {
    /// Activate packing mode.
    #[arg(long, help = "Activate packing mode.")]
    pub pack: bool,

    /// Activate unpacking mode.
    #[arg(long, help = "Activate unpacking mode.")]
    pub unpack: bool,

    /// (Required for packing) The path to the source file or directory to be compressed.
    #[arg(short, long, value_name = "SOURCE", required_if_eq("pack", "true"))]
    pub source: Option<String>,

    /// (Required for packing) The name or path of the output .rz archive file.
    #[arg(short, long, value_name = "OUTPUT", required_if_eq("pack", "true"))]
    pub output: Option<String>,

    /// (Required for unpacking) The path to the .rz archive file to be decompressed.
    #[arg(short, long, value_name = "ARCHIVE", required_if_eq("unpack", "true"))]
    pub archive: Option<String>,

    /// (Optional for unpacking) The destination directory for extraction. Defaults to the current directory.
    #[arg(short, long, value_name = "DESTINATION")]
    pub destination: Option<String>,

    /// (Optional) Password for encryption or decryption.
    #[arg(short, long, value_name = "PASSWORD")]
    pub password: Option<String>,
}
