# Radhe AI 🕉️

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Project Status: Active](https://img.shields.io/badge/Project%20Status-Active-green.svg)](PROJECT_STATUS.md)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)](PROJECT_GUIDE.md)

A state-of-the-art, lightning-fast, **100% offline terminal AI assistant** tailored for college students, hostel users, and beginner programmers. 

Radhe AI delivers high-performance coding support, academic revision guides, and syntax debugging without requiring an internet connection, account logins, api keys, or subscriptions. All model inference runs strictly locally on your CPU.

---

## 📖 Navigation Portal

Explore detailed architectural specifications, contributor workflows, and user guides:

* 🏗️ **[Architectural Design](ARCHITECTURE.md)** — Deep dive into Rust execution wrappers, prompt engineering templates, and post-inference delimiters.
* 🛠️ **[Feature & Prompt Reference](FEATURE_LIST.md)** — Comprehensive list of all CLI switches (`--code`, `--explain`, `--notes`, `--fix`), parameters, and rules.
* 🎓 **[Student & Onboarding Guide](PROJECT_GUIDE.md)** — Onboarding steps, daily student workflows, and local troubleshooting procedures.
* ⚡ **[Developer Contributing Guide](CONTRIBUTING.md)** — local cargo toolchains, compilation flags, linter settings, and PR workflow checklists.
* 📈 **[Project Status & Roadmaps](PROJECT_STATUS.md)** — Progress board tracking MVP completion (`v0.1.0`) and the installer milestone (`v0.2.0`).
* 🤖 **[Agent Integration Guidelines](AGENT_DOCS.md)** — Developer advice for wrapping or programmatically scripting Radhe AI into other AI extensions or IDE tasks.
* 🛡️ **[Security & Privacy Policy](SECURITY.md)** — Details on offline execution boundaries and how to report vulnerabilities.
* 🤝 **[Community Code of Conduct](CODE_OF_CONDUCT.md)** — Contributor expectations and inclusive environment policies.
* 📄 **[MIT License](LICENSE)** — Core terms and legal permissions.
* 📋 **[Changelog](CHANGELOG.md)** — Structural record of all releases and added features.

---

## ✨ Features at a Glance

* 💻 **`radhe --code` (Pure Code Generation)**: Writes compilable, raw source code blocks directly inside the terminal without conversational preambles.
* 🎓 **`radhe --explain` (Beginner Explanations)**: Distills complex computer science concepts into exactly 5 intuitive, single-sentence bullet points.
* 📝 **`radhe --notes` (Academic Revision Notes)**: Compiles 6 highly concise, 10-word fact sheets designed for exam preparation.
* 🔧 **`radhe --fix` (Case-Insensitive Compiler Debugger)**: Automatically detects programming language formats, pre-filters comments to strip hint markers, fixes syntax errors, and outputs corrected scripts.
* 💬 **Interactive REPL Mode**: Type `radhe` with no flags to launch a beautiful colorized prompt supporting `/clear` and `/exit` commands, shortcut prefixes, and robust `Ctrl+C` interrupt safety.
* 🩺 **`radhe doctor`**: Built-in system health utility that automatically checks local paths and model setups.

---

## 🚀 Getting Started

### 1. Installation Requirements
Before running Radhe AI, ensure the following local items are configured:
* **The GGUF Model**: Quantized Qwen2 (`qwen2.gguf`) must be downloaded and stored at:
  - **Windows**: `C:\Users\<username>\.radhe\models\qwen2.gguf`
  - **Linux/macOS**: `~/.radhe/models/qwen2.gguf`
* **Inference Library**: `llama-completion.exe` (or `llama-completion` binary on UNIX-based systems) must be present in your environment `PATH`.

### 2. Initializing & Checking Environment
Create your local storage directories automatically:
```bash
radhe init
```

Verify your environment dependencies:
```bash
radhe doctor
```

### 3. Usage Examples

#### Run general prompts:
```bash
radhe "what is a segmentation fault in C?"
```

#### Generate compilable Rust code:
```bash
radhe --code "binary search function in rust"
```

#### Explain a database concept:
```bash
radhe --explain "ACID transactions"
```

#### Create notes on networking protocols:
```bash
radhe --notes "TCP vs UDP handshakes"
```

#### Fix bugs in a C program:
```bash
radhe --fix homework.c
```

---

## 🛠️ Local Development

Radhe AI is engineered in highly optimized Rust. To compile it from source:

1. Setup your stable Rust toolchain.
2. Compile and run:
   ```bash
   cargo build
   cargo run -- --explain "pointer arithmetic"
   ```
3. Build highly optimized release binaries (with Link-Time Optimization and Symbol Stripping active):
   ```bash
   cargo build --release
   ```
   *The resulting executable is generated at `target/release/radhe` (or `radhe.exe` on Windows) and measures `< 1.5MB`.*

---

## 📄 License

This project is licensed under the terms of the [MIT License](LICENSE).
