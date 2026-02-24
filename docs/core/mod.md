# mod.rs Documentation (core)

This document provides an overview of `src/core/mod.rs`, which defines the `core` module for the Raze archiving application.

## Overview

This module encapsulates the fundamental logic for the Raze archiving application, providing the essential capabilities for both compression and decompression of `.rz` archives. It acts as the central processing unit for the application's primary functionalities, orchestrating how files are archived and extracted.

## Sub-modules

The `core` module is organized into two main sub-modules:

*   `compress`: This sub-module is dedicated to handling the creation of `.rz` archives. It provides the necessary functions and logic to pack files or directories into a compressed archive format.
*   `decompress`: This sub-module focuses on managing the extraction of contents from `.rz` archives. It contains the logic required to unpack compressed archives, restoring files and directories to their original state.

These sub-modules integrate with external crates such as `tar` for archiving and `zstd` for high-performance compression, thereby offering a robust and efficient archiving solution.
