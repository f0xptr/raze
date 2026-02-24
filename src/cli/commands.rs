//! # CLI Commands Module
//!
//! This module defines the available subcommands that the Raze CLI application supports.
//! It uses `clap`'s `Subcommand` derive to enumerate and describe the primary operations
//! a user can perform, such as `pack` for compression and `unpack` for decompression.
//!
//! Each subcommand variant includes its specific arguments and a clear description
//! of its purpose, enhancing the CLI's usability and self-documentation.

use clap::Subcommand;

/// Enumerates the core operations that the Raze CLI can perform.
///
/// This `enum` defines the distinct functionalities accessible via the command line.
/// Each variant represents a subcommand that users can invoke, along with its
/// specific set of parameters required for execution.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Compresses a file or directory into a `.rz` archive.
    ///
    /// The `pack` subcommand takes a source path (file or directory) and an
    /// output path for the new `.rz` archive. It uses Zstandard compression
    /// and Tar archiving to create efficient archives.
    Pack {
        /// The path to the source file or directory to be compressed.
        ///
        /// This can be an absolute or relative path to the content intended
        /// for archiving.
        #[arg(short, long, value_name = "SOURCE")]
        source: String,

        /// The name or path of the output `.rz` archive file.
        ///
        /// If the `.rz` extension is omitted, it will be automatically appended.
        #[arg(short, long, value_name = "OUTPUT")]
        output: String,
    },
    /// Decompresses a `.rz` archive into the current or a specified directory.
    ///
    /// The `unpack` subcommand requires the path to an existing `.rz` archive
    /// and an optional destination directory where the contents will be extracted.
    Unpack {
        /// The path to the `.rz` archive file that needs to be decompressed.
        ///
        /// This must be a valid path to an existing Raze archive.
        #[arg(short, long, value_name = "ARCHIVE")]
        archive: String,

        /// The destination directory where the archive's contents will be extracted.
        ///
        /// If not specified, the archive will be extracted into the current working directory.
        #[arg(short, long, value_name = "DESTINATION", default_value = ".")]
        destination: String,
    },
}
