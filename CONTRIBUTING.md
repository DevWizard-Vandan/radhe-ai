# Contributing to Radhe AI

Thank you for your interest in contributing to Radhe AI! We welcome code fixes, performance updates, and improved prompts from students and developers alike.

---

## 1. Development Setup

### System Prerequisites
To compile and edit Radhe AI locally, you will need:
* The Rust toolchain (stable compiler `rustc` and package manager `cargo`).
* The local inference library `llama-completion.exe` inside your PATH.
* The local Qwen2 model `qwen2.gguf` saved in `~/.radhe/models/`.

### Cloning and Building
1. Clone the repository:
   ```bash
   git clone https://github.com/DevWizard-Vandan/radhe-ai.git
   cd radhe-ai
   ```
2. Build the project in debug mode:
   ```bash
   cargo build
   ```
3. Compile highly optimized, lightweight binaries with link-time optimization (LTO) and symbol stripping enabled:
   ```bash
   cargo build --release
   ```

---

## 2. Formatting & Coding Style

We adhere to standard Rust formatting guidelines to maintain high readability across the project:

* **Format Check**: Always format your code before creating a pull request:
  ```bash
  cargo fmt --all -- --check
  ```
* **Linter Warnings**: Run clippy to check for common antipatterns and fixable compiler warnings:
  ```bash
  cargo clippy --all-targets -- -D warnings
  ```
* **Binary Footprint**: Radhe AI is designed as a tiny terminal helper. When introducing new dependencies in `Cargo.toml`, ensure they do not excessively bloat the final compiled binary. Keep use of external crates minimal and prefer standard library solutions.

---

## 3. Pull Request Guidelines

1. **Fork & Branch**: Create a new branch for your feature or bug fix:
   ```bash
   git checkout -b feature/my-cool-feature
   ```
2. **Commit Messages**: Write structured, clear commit messages. We follow conventional commit standards, e.g.:
   - `feat: add new CLI argument`
   - `fix: correct prompt stripping delimiter`
   - `docs: update CONTRIBUTING guides`
3. **Verify Locally**: Run `cargo check` and execute local manual runs of REPL and `--fix` modes before pushing.
4. **Push and Open PR**: Push to your fork and submit a PR to the `main` branch. Provide a clear description of your modifications and a verification walkthrough.
