# Project Status & Milestones

This document highlights the current developmental health, active milestones, and release stages of Radhe AI.

---

## Active Release Status

* **Current Stable Version**: `v0.1.1` (Interactive quiz, model configuration & selection, doctor diagnostics, models, and update subcommands)
* **Status**: :white_check_mark: Active development / v0.1.1 successfully launched.

---

## Milestones Dashboard

### :checkered_flag: Milestone 0.1.1: Advanced CLI Options & Quiz Engine (Completed)
* [x] **Interactive CS Quiz Engine (`--quiz` / `--count`)**: Formulates custom computer science student exams and evaluates student input case-insensitively.
* [x] **Configuration System (`config.toml`)**: Auto-generates configuration file `~/.radhe/config.toml` on startup and parses preferences.
* [x] **CLI Flag Overrides**: Priority resolution: CLI `--model` / `--max-tokens` > `config.toml` > defaults.
* [x] **Model Subcommand (`radhe models`)**: Auto-scans directory size, outputs MB, and highlights active `.gguf` file.
* [x] **Self-Update Engine (`radhe update`)**: Automated connection check, binary replacement, and file renaming on Windows.

### :checkered_flag: Milestone 0.1.0: Core Offline Engine (Completed)
* [x] **Lightweight Rust CLI Harness**: Built with `clap` and optimized release settings.
* [x] **Subprocess local runner**: Connects to `llama-completion.exe` with precise temperature thresholds.
* [x] **Specialized Prompts**:
  - `--code`: Generates compilable code blocks with prompt boundaries.
  - `--explain`: Short 5-bullet conceptual definitions.
  - `--notes`: Quick 6-bullet academic revision notes.
* [x] **Compiler & Debugger `--fix` Mode**: Uses case-insensitive extensions to mimic specific compilers, and strips comment templates.
* [x] **Interactive REPL Mode**: Colored welcome headers, green prompt, standard commands (`/exit`, `/clear`), and `Ctrl+C` interrupt handlers.
* [x] **Environment Diagnostics**: `doctor` mode checks paths and validates local model downloads.

### :rocket: Milestone 0.2.0: The Student Installer (In Progress)
* [ ] **Automated Powershell Installer**: Create `install.ps1` to download `llama-completion.exe` and `qwen2.gguf` automatically to the standard directories.
* [ ] **Automated Shell Installer**: Create `install.sh` for Linux/macOS compatibility.
* [ ] **Pre-compiled Binary Bundles**: Release pre-built platform-dependent packages via GitHub Releases.
* [ ] **Unified Setup Directory**: Establish directory creation during install inside the user's home folder `~/.radhe`.

### :bulb: Milestone 0.3.0: Hindi Support & Editor Integrations (Planned)
* [ ] **Multilingual Prompt Templates**: Add Hindi-English (Hinglish) support for explanation and student notes modes.
* [ ] **VS Code Extension**: Lightweight extension to invoke the local model using keyboard shortcut bindings.
* [ ] **Vim/Neovim Plugin**: Simple Lua script to pipe marked visual blocks straight through `radhe --fix`.

---

## Technical Health Metrics

* **Code Coverage**: Cargo integration testing planned for `v0.1.5`.
* **Binary Size**: Optimized at `< 1.5MB` via compilation profiling (`lto = true`, `strip = true`).
* **Memory Usage**: Under `20MB` runtime footprint (excluding the quantized GGUF model memory in the `llama` backend).
