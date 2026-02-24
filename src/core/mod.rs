//! # Core Archiving Logic Module
//!
//! This module encapsulates the fundamental logic for Raze, providing the
//! capabilities for both compression and decompression of `.rz` archives.
//! It serves as the central processing unit for the application's primary
//! functionalities.
//!
//! The module is divided into two main sub-modules:
//! - `compress`: Handles the creation of `.rz` archives from files or directories.
//! - `decompress`: Manages the extraction of contents from `.rz` archives.
//!
//! These sub-modules work in conjunction with external crates like `tar` for
//! archiving and `zstd` for high-performance compression, offering a robust
//! and efficient archiving solution.
pub mod compress;
pub mod decompress;
