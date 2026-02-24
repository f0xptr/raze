# mod.rs Documentation (cli)

This document provides an overview of `src/cli/mod.rs`, which defines the `cli` module for the Raze archiving utility.

## Overview

This module functions as the central point for managing all command-line interactions within the Raze archiving utility. It integrates the `clap` crate to deliver robust argument parsing capabilities, efficient subcommand dispatching, and automated generation of comprehensive help messages.

## Structure

The module is meticulously structured to maintain a clear separation between the definitions of command-line arguments (found in `args.rs`) and their specific command implementations (located in `commands.rs`). This design approach ensures a clean, user-friendly, and easily maintainable interface.

## Sub-modules

*   `args`: This sub-module defines the command-line argument structure, including the main `RazeArgs` struct and its subcommands, using `clap` for parsing.
*   `commands`: This sub-module contains the specific implementations for each command that the Raze CLI supports, detailing the actions performed for `pack`, `unpack`, etc.
