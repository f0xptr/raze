# Contributing to Raze

We welcome contributions to Raze! By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

Before contributing, please read the following guidelines.

## How Can I Contribute?

### Reporting Bugs

If you find a bug, please open an [issue](.github/ISSUE_TEMPLATE/bug_report.md) on GitHub. When reporting a bug, please include:

-   A clear and concise description of the bug.
-   Steps to reproduce the behavior.
-   Expected behavior.
-   Screenshots (if applicable).
-   Your environment details (OS, Raze version, Rust version).
-   Any additional context.

### Suggesting Enhancements

If you have an idea for a new feature or an improvement, please open an [issue](.github/ISSUE_TEMPLATE/feature_request.md) on GitHub. When suggesting an enhancement, please include:

-   A clear and concise description of the feature or enhancement.
-   The problem it solves.
-   Any alternative solutions you've considered.
-   Additional context or screenshots.

### Asking Questions

If you have a question about Raze, please open an [issue](.github/ISSUE_TEMPLATE/question.md) on GitHub.

## Your First Contribution

If you're looking for an easy way to get started, look for issues with the `good first issue` label.

## Pull Request Guidelines

1.  **Fork the repository** and create your branch from `main`.
2.  **Ensure your code adheres to the project's coding style.** You can run `cargo fmt` to automatically format your code.
3.  **Ensure your changes pass all tests.** Run `cargo test` locally.
4.  **Write clear, concise commit messages.**
5.  **Update documentation** if your changes introduce new features or modify existing behavior.
6.  **Create a pull request** to the `main` branch.
    -   Fill out the [pull request template](.github/PULL_REQUEST_TEMPLATE.md) completely.
    -   Reference any related issues (e.g., `Fixes #123`).

## Development Setup

To set up your development environment:

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/f0xptr/raze.git
    cd raze
    ```
2.  **Install Rust:** If you don't have Rust installed, follow the instructions on [rustup.rs](https://rustup.rs/).
3.  **Build the project:**
    ```bash
    cargo build
    ```
4.  **Run tests:**
    ```bash
    cargo test
    ```
5.  **Check formatting and linting:**
    ```bash
    cargo fmt --check
    cargo clippy
    ```

Thank you for contributing to Raze!
