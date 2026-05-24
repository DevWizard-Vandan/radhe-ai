# Radhe AI

A tiny offline terminal AI assistant for students.

Radhe AI is designed for college students, hostel users, and beginner programmers who need quick coding help without internet, logins, subscriptions, or heavy setup.

## Vision

- Offline-first after installation
- Tiny and fast
- Terminal-native
- Beginner-friendly
- Optimized for coding and academic help

## MVP Commands

```bash
radhe --code "bubble sort in c"
radhe --explain "binary tree"
radhe --notes "operating system scheduling"
radhe --fix main.c
radhe "what is stack overflow"
```

## Planned Stack

- Rust for CLI
- llama.cpp for local inference
- GGUF quantized models
- Qwen2 0.5B / TinyLlama for small-device support
- GitHub Releases for distribution

## Project Structure

```text
radhe-ai/
├── .github/
│   ├── ISSUE_TEMPLATE/
│   └── workflows/
├── installer/
├── src/
│   └── main.rs
├── Cargo.toml
└── README.md
```

## Local Development

```bash
cargo build
cargo run -- --code "linked list in c"
```

Radhe currently expects `llama-cli` from `llama.cpp` to be available in your PATH and a GGUF model to exist at `./models/qwen2.gguf`.

## Roadmap

- [ ] Basic CLI modes
- [ ] Prompt templates
- [ ] Model auto-download
- [ ] Windows installer
- [ ] Linux/macOS installer
- [ ] Interactive REPL
- [ ] VS Code integration
- [ ] Hindi + English support

## License

MIT
