# logger.rs Documentation

This document provides an overview of `src/utils/logger.rs`, which is responsible for initializing and configuring the application's logging environment.

## Overview

This module handles the initialization and configuration of the application's logging environment. It offers a straightforward function to set up `env_logger`, enabling flexible control over log output through environment variables.

## Functions

### `fn init()`

Initializes the application's logger.

This function configures `env_logger` to display log messages in the console. By default, it sets the log level to `info`, meaning messages at `info`, `warn`, and `error` levels will be visible.

The default log level can be overridden by setting the `RUST_LOG` environment variable before executing the application. For instance:

*   `RUST_LOG=debug ./raze` will activate debug-level logging.
*   `RUST_LOG=raze=trace ./raze` will specifically enable trace-level logging for modules within the `raze` crate.

**Panics:**
This function is designed not to panic. Repeated calls to `init` will not cause issues, as `env_logger` gracefully manages re-initialization (subsequent calls will perform no action if logging is already active).
