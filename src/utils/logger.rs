//! # Logging Utility Module
//!
//! This module is responsible for initializing and configuring the application's
//! logging environment. It provides a simple function to set up `env_logger`,
//! allowing for flexible control over log output via environment variables.

use env_logger::Env;

/// Initializes the application's logger.
///
/// This function sets up the `env_logger` to output log messages to the console.
/// By default, it configures the log level to `info`, meaning messages at
/// `info`, `warn`, and `error` levels will be displayed.
///
/// The default log level can be overridden by setting the `RUST_LOG` environment
/// variable before running the application. For example:
///
/// - `RUST_LOG=debug ./raze` will enable debug-level logging.
/// - `RUST_LOG=raze=trace ./raze` will enable trace-level logging specifically
///   for modules within the `raze` crate.
///
/// # Panics
///
/// This function does not panic. Multiple calls to `init` will not cause issues
/// as `env_logger` handles re-initialization gracefully (subsequent calls
/// will silently do nothing if logging is already initialized).
pub fn init() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
}
