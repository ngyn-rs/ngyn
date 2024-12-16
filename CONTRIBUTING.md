# Contributing to ngyn

## Welcome!

First off, thank you for considering contributing to ngyn! It's people like you that make ngyn such a great tool.

## Code of Conduct

Please note that this project is released with a [Contributor Code of Conduct](CODE_OF_CONDUCT.md). By participating in this project, you agree to abide by its terms.

## How Can I Contribute?

### Reporting Bugs

- Use the GitHub Issues section
- Provide a clear and descriptive title
- Describe the exact steps to reproduce the problem
- Include your operating system, Rust version, and ngyn version or commit hash

### Suggesting Enhancements

- Open a GitHub Issue
- Provide a clear and detailed explanation of the suggestion
- Explain why this enhancement would be useful to most ngyn users

### Pull Requests

1. Fork the repository
2. Create a new branch for your feature or bugfix
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. Make your changes
4. Write or update tests
5. Ensure all tests pass
   ```bash
   cargo test
   ```
6. Commit your changes with a descriptive commit message
7. Push to your fork and submit a pull request

### Development Setup

1. Install Rust (https://rustup.rs/)
2. Clone the repository
   ```bash
   git clone https://github.com/ngyn-rs/ngyn.git
   cd ngyn
   ```
3. Install development dependencies
   ```bash
   cargo build
   ```

## Development Workflow

- Follow Rust's official style guidelines
- update examples if necessary
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes with no warnings
- Update Changelog for each crate that is modified
- Write tests for new features and bug fixes

## Questions?

If you have any questions, please open an issue or reach out to the maintainers.

## Thank You!

Your contributions make open source amazing. Thank you for your help!