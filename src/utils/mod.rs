//! # Utility Module
//!
//! This module provides a collection of utility functions, custom error definitions,
//! and logging setup for the Raze archiving application. It consolidates common
//! functionalities that are reused across different parts of the codebase,
//! promoting modularity and maintainability.
//!
//! The module is structured into the following sub-modules:
//! - `errors`: Defines a comprehensive set of custom error types (`RazeError`)
//!   used for consistent error reporting throughout the application.
//! - `logger`: Handles the initialization and configuration of the application's
//!   logging system, allowing for configurable diagnostic output.
pub mod errors;
pub mod logger;
