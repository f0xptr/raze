# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2026-03-03

### Added
- **Secure Encryption**: Added password-based encryption using AES-256-GCM with a key derived via Argon2id.
- **CLI Tests**: Added new integration tests for the command-line interface.

### Changed
- **CLI Refactoring**: Refactored the CLI to use flags (`--pack`, `--unpack`) instead of subcommands, providing a single, unified help message.
- **Updated Documentation**: Updated all documentation, including the `README.md`, to reflect the new CLI structure and encryption features.

## [1.0.0] - 2026-02-25

### Added
- **Core Functionality**: Initial project setup including core compression and decompression logic.
- **Command-Line Interface**: Implemented basic CLI with `pack` and `unpack` commands.
- **Robustness**: Basic error handling and logging mechanisms.
- **Documentation**: Initial internal code comments and project documentation.
- **Developer Experience**:
    - CI/CD workflow using GitHub Actions.
    - Issue and Pull Request templates for structured contributions.
    - Comprehensive contribution guidelines, contributors list, and code of conduct.
    - EditorConfig for consistent coding styles.
    - Formatting and linting script for code quality.
