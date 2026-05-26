# Radhe AI ⚡

> A tiny offline terminal AI assistant for students.  
> No internet. No login. No API keys. Just type and get answers.

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

## Install (Windows)

Open PowerShell and run:

```powershell
irm https://raw.githubusercontent.com/DevWizard-Vandan/radhe-ai/main/installer/install.ps1 | iex
```

That's it. The installer will:
- Download `radhe.exe` (~640 KB)
- Download `llama-completion.exe` (inference engine)
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
Radhe AI v0.1.0 — Offline Terminal Assistant
>>> --code "stack in c"
>>> --explain "pointers"
>>> /exit
```

---

## System Requirements

| Requirement | Minimum |
|---|---|
| OS | Windows 10/11 x64 |
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
