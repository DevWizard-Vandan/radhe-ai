# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.3.0] — 2026-05-26

### Added
- **File Summarization Mode (`--summarize`)**:
  - Summarizes student notes into 5 structured, concise bullet facts starting with a dash.
  - Automatically truncates notes exceeding 3000 characters to stay within model context bounds.
  - Generates loading indicators (`Summarizing {filename}...`) and handles missing/empty file bounds cleanly.
- **Default Model Upgrade**:
  - Upgraded the default offline quantized LLM from `qwen2` (0.5B) to the advanced `Qwen2.5-Coder-1.5B-Instruct-Q4_K_M.gguf` (1.04GB).
  - Maintained full backwards compatibility, enabling the 0.5B model via the `--model qwen-0.5b` override.
- **Interactive Quiz Mode (`--quiz` / `--count`)**:
  - Generates custom computer science student exam multiple-choice questions.
  - Features real-time standard input parsing and matching (case-insensitive ticks and crosses).
  - Skips scoring gracefully if answer keys or valid labels are missing.
- **Model Selection & Configuration System**:
  - Adds `~/.radhe/config.toml` auto-generation and dynamic parsing.
  - Implements three-tier priority resolution: CLI `--model` > `config.toml` > hardcoded default.
  - Allows `--max-tokens` CLI overrides and file configuration.
- **Dynamic Model Subcommand (`radhe models`)**:
  - Lists downloaded `.gguf` model files under `~/.radhe/models/`.
  - Shows exact sizes in MB and highlights the active model with `*` and `[active]` labels.
- **Upgraded Environment Diagnostics**:
  - `doctor` subcommand dynamically validates and locates the configured active GGUF model path.
- **Self-Update Engine (`radhe update`)**:
  - Checks for the latest release via the GitHub API by querying latest release details.
  - Handles update validation, binary download, and renaming of standard binaries for zero-interruption updates on Windows.

## [0.1.0] — 2026-05-25

### Added
- **Interactive REPL Mode**:
  - Automatically launches when the CLI is run with no arguments or flags.
  - Features beautiful, state-of-the-art colorized prompts (cyan welcome header, yellow guidelines, green `>>> ` marker via the `colored` crate).
  - Handles `Ctrl+C` cleanly with a warm cyan farewell message.
  - Supports inline commands `/clear` (clears screen) and `/exit` (exits REPL).
  - On-the-fly shortcut support (prefixes `--code`, `--explain`, `--notes` parse automatically inside the loop).
- **Compiler & Debugger `--fix` Mode**:
  - Overhauled file logic to automatically accept file path strings.
  - Built-in path validation returning precise error details if files are missing.
  - Case-insensitive, robust file extension mapper (`.c` -> `C`, `.cpp` -> `C++`, `.py` -> `Python`, `.rs` -> `Rust`, `.java` -> `Java`).
  - Pre-inference hint stripper: automatically filters out all `// bug:` or `# bug:` comment markers before model submission to protect exercise integrity.
- **`radhe --code` Mode**: Generates compilable code blocks matching specified natural language requests (max 300 tokens).
- **`radhe --explain` Mode**: Explains concepts in exactly 5 simple, one-sentence beginner bullet points (max 200 tokens).
- **`radhe --notes` Mode**: Synthesizes 6 short exam-style student notes limited to 10 words per bullet (max 150 tokens).
- **`radhe doctor`**: Static system health checker verifying `llama-completion.exe` inside PATH and model GGUF presence.
- **`radhe init`**: Standard directory initializer setting up expected `models` and `installer` target directories.
- **Output Echo-Stripping Engine**: Two-tiered splitting framework (`### RESPONSE:`, `FIXED CODE:\n`) combined with platform-agnostic escape normalization to strip prompt headers perfectly.
- **Syntax and Formatting Post-Processors**: Markdown fence block cleaner, recursive `[end of text]` token vacuum, and custom stop keywords.
