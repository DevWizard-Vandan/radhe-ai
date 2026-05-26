# Radhe AI ⚡

> A tiny offline terminal AI assistant for students.  
> No internet. No login. No API keys. Just type and get answers.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Project Status: Active](https://img.shields.io/badge/Project%20Status-Active-green.svg)](PROJECT_STATUS.md)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)](PROJECT_GUIDE.md)

---

## Why Radhe AI?

Most AI tools require:
- ☁️ Internet connection
- 🔑 API keys or accounts
- 💻 Heavy installations
- 💳 Subscriptions

**Radhe AI requires none of that.**

Built for B.Tech/Diploma students with weak WiFi, low-end laptops, and no budget.

---

## Features

| Command | Description |
|---|---|
| `radhe --code "bubble sort in c"` | Generate compilable code instantly |
| `radhe --explain "recursion"` | Get beginner-friendly explanations |
| `radhe --notes "OS scheduling"` | Generate bullet-point study notes |
| `radhe --fix main.c` | Debug and fix broken code files |
| `radhe` | Launch interactive REPL session |
| `radhe doctor` | Check installation health |

---

## Install
### Windows (PowerShell)
```powershell
irm https://raw.githubusercontent.com/DevWizard-Vandan/radhe-ai/main/installer/install.ps1 | iex
```
### Linux / macOS (bash)
```bash
curl -fsSL https://raw.githubusercontent.com/DevWizard-Vandan/radhe-ai/main/installer/install.sh | bash
```
Both installers will:
- Download the `radhe` binary (~800 KB)
- Download `llama-completion` (inference engine)
- Download `qwen2.gguf` AI model (~400 MB)
- Add everything to PATH automatically

Restart your terminal, then run:

```powershell
radhe --code "hello world in c"
```

---

## Usage Examples

### Generate Code

```bash
$ radhe --code "linked list in c"

#include <stdio.h>
#include <stdlib.h>

struct Node {
    int data;
    struct Node* next;
};
```

### Explain a Concept  

```bash
$ radhe --explain "binary search tree"

A BST stores values where left < root < right
Search runs in O(log n) on average
```

### Fix Broken Code

```bash
$ radhe --fix main.c

#include <stdio.h>
int main() {
    int x = 10; // semicolon fixed
}
```

### Interactive REPL

```text
$ radhe
Radhe AI v0.2.0 — Offline Terminal Assistant
>>> --code "stack in c"
>>> --explain "pointers"
>>> /exit
```

---

## System Requirements

| Requirement | Minimum |
|---|---|
| OS | Windows 10/11 x64 OR Ubuntu 20.04+ x64 |
| RAM | 2 GB free |
| Storage | 600 MB |
| Internet | Only during install |

---

## How It Works

```text
radhe CLI (Rust)
      ↓
Prompt Engineering
      ↓
llama-completion.exe (llama.cpp)
      ↓
qwen2.gguf (Qwen2.5-Coder 0.5B, runs on CPU)
      ↓
Response printed to terminal
```

No cloud. No servers. Everything runs on your laptop.

---

## Roadmap

- [x] `--code` mode
- [x] `--explain` mode  
- [x] `--notes` mode
- [x] `--fix` mode
- [x] Interactive REPL
- [x] Windows installer
- [ ] Linux support
- [ ] Colored terminal output
- [ ] VS Code extension
- [ ] Hindi + English mode
- [ ] Offline DSA tutor

---

## Contributing

Pull requests welcome! Please open an issue first to discuss what you'd like to change.

---

## License

MIT — free to use, modify, and distribute.

---

*Built with ❤️ for Indian students who just want to code.*
