# Project Status & Milestones

This document highlights the current developmental health, active milestones, and release stages of Radhe AI.

---

## Active Release Status

* **Current Stable Version**: `v0.5.0` (Persistent chat, quiz file generation, models, update, and summarize subcommands)
* **Status**: :white_check_mark: Active development / v0.5.0 successfully launched.

---

## Milestones Dashboard

### :checkered_flag: Milestone 0.4.0: Persistent Chat & Quiz From File (Completed)
* [x] **Persistent chat mode via `--chat`**: 6-turn rolling context window to preserve interactive dialogue history.
* [x] **ChatML prompt format for Qwen instruct models**: Native support for delimiters (`<|im_start|>`/`<|im_end|>`) matching Qwen instruct models.
* [x] **Fixed context history passing across turns**: Robust prompt stripping preventing conversation turn boundary hallucination.
* [x] **`--quiz-file`**: Generate quiz questions from student notes files.
* [x] **`--version` flag**: Prints version + active model instantly.
* [x] **Enhanced doctor**: Version header, active model validation, colored output.

### :checkered_flag: Milestone 0.3.0: File Summarization & Self-Updates (Completed)
* [x] **File Summarization (`--summarize`)**: Truncates inputs to 3000 characters and generates structured 5-bullet summary notes.
* [x] **Self-Update Engine (`radhe update`)**: PowerShell-based live connection check, secure credential retrieval, and automatic Windows executable swapping.
* [x] **Default Model Upgrade**: Lifted default offline LLM model from `0.5B` to the advanced `Qwen2.5-Coder 1.5B`.
* [x] **Interactive CS Quiz Engine (`--quiz` / `--count`)**: Formulates custom computer science student exams and evaluates student input case-insensitively.
* [x] **Configuration System (`config.toml`)**: Auto-generates configuration file `~/.radhe/config.toml` on startup and parses preferences.
* [x] **CLI Flag Overrides**: Priority resolution: CLI `--model` / `--max-tokens` > `config.toml` > defaults.
* [x] **Model Subcommand (`radhe models`)**: Auto-scans directory size, outputs MB, and highlights active `.gguf` file.

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

### :checkered_flag: Milestone 0.2.0: The Student Installer (Completed)
* [x] **Automated PowerShell installer (`install.ps1`)**: Downloads `radhe.exe`, `llama.cpp`, advanced Qwen 1.5B model, updates PATH.
* [x] **Pre-compiled binary bundles**: pre-compiled packages released via GitHub Releases (Windows + Linux).
* [x] **Unified `~/.radhe` directory structure**: Establishes automatic setup and scanner bounds in `~/.radhe`.
* [ ] **Automated Shell Installer (`install.sh`)**: Create `install.sh` for Linux/macOS compatibility.

### :bulb: Milestone 0.5.0: Hindi Support & Editor Integrations (Planned)
* [ ] **Multilingual Prompt Templates**: Add Hindi-English (Hinglish) support for explanation and student notes modes.
* [ ] **VS Code Extension**: Lightweight extension to invoke the local model using keyboard shortcut bindings.
* [ ] **Vim/Neovim Plugin**: Simple Lua script to pipe marked visual blocks straight through `radhe --fix`.

---

## Technical Health Metrics

* **Code Coverage**: Cargo integration testing planned for `v0.1.5`.
* **Binary Size**: Optimized at `< 1.5MB` via compilation profiling (`lto = true`, `strip = true`).
* **Memory Usage**: Under `20MB` runtime footprint (excluding the quantized GGUF model memory in the `llama` backend).
