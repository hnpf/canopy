# Contributing

We really welcome contributions to Canopy! This document provides guidelines for contributing to the project.

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to deprecated@virex.lol

## How to Contribute

### Reporting Bugs

- Use the GitHub issue tracker to report bugs.
- Describe the bug and include steps to reproduce it.
- Include your operating system and version of Canopy.

### Suggesting Features

- Use the GitHub issue tracker to suggest features.
- Provide a clear description of the feature and why it would be useful! 

### Contributing Code

1. Fork the repository on GitHub.
2. Clone your fork locally: `git clone https://github.com/hnpf/canopy.git`
3. Create a new branch for your changes: `git checkout -b <FEATURENAME>`
4. Make your changes.
5. Make sure the code compiles: `cargo build`
6. Run tests: `cargo test`
7. Format the code: `cargo fmt`
8. Check for linting issues: `cargo clippy`
9. Commit your changes: `git commit -am 'Add some feature'`
10. Push to your fork: `git push origin <feature-name>`
11. Create a pull request on GitHub.

### Pull Request Guidelines

- Provide a clear description of what your PR does.
- Reference any related issues.
- Ensure all tests pass.
- Follow the existing code style.

## Development Setup

1. Install Rust: https://rustup.rs/
2. Clone the repository: `git clone https://github.com/hnpf/canopy.git`
3. Build the project: `cargo build`
4. Run tests: `cargo test`

## Code Style

- Follow the Rust style guidelines.
- Use `cargo fmt` to format code.
- Use `cargo clippy` to check for common mistakes.

## License

By contributing to Canopy, you agree that your contributions will be licensed under the GPL 3.0 License.