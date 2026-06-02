# Radhe AI 🎓
> Your offline AI study assistant — runs entirely on your laptop, no internet required.

## Features
| Command | What it does |
|---|---|
| `radhe --code "..."` | Generate code from a prompt |
| `radhe --explain "..."` | Explain a concept clearly |
| `radhe --notes "..."` | Generate study notes |
| `radhe --fix "..."` | Fix broken code |
| `radhe --quiz "..."` | Get quiz questions on a topic |
| `radhe --quiz-file <file>` | Generate quiz questions from a notes file |
| `radhe --summarize <file>` | Summarize a text file into 5 bullets |
| `radhe --chat` | Start a persistent conversation |
| `radhe --lang hi` | Response in Hindi (also: `hinglish`) |
| `radhe --set-lang hinglish` | Set default language permanently |
| `radhe --mode <mode>` | Set study mode (`normal`, `exam`, `revision`) |
| `radhe --difficulty <diff>` | Set quiz difficulty (`easy`, `medium`, `hard`) |
| `radhe --set-mode <mode>` | Save default study mode permanently |
| `radhe --set-difficulty <diff>` | Save default quiz difficulty permanently |
| `radhe update` | Self-update to the latest version |
| `radhe models` | List installed models |
| `radhe doctor` | Check installation health |

## Installation
### Windows
Open PowerShell and run:
```powershell
irm https://raw.githubusercontent.com/DevWizard-Vandan/radhe-ai/main/install.ps1 | iex
```
This installs `radhe.exe` and downloads the default model (~1GB). Restart your terminal after install.

### Linux / macOS
Open your terminal and run:
```bash
curl -fsSL https://raw.githubusercontent.com/DevWizard-Vandan/radhe-ai/main/install.sh | bash
```
This installs `radhe` and downloads the default model (~1GB). Restart your terminal after install.

## Usage Examples
```powershell
# Explain a concept
radhe --explain "what is a binary search tree"
# Summarize your lecture notes
radhe --summarize notes.txt
# Chat session
radhe --chat
# Update to latest version
radhe update
```

## Subject Packs
Radhe ships with 3 starter packs that tune its responses for specific subjects:
| Pack | Command | Coverage |
|------|---------|----------|
| Math | `radhe --pack math` | Algebra, Calculus, Stats, Trig |
| Computer Science | `radhe --pack cs` | DSA, OS, DBMS, Networking |
| Science | `radhe --pack science` | Physics, Chemistry, Biology |

List installed packs:
```bash
radhe --list-packs
```
Install a custom pack by dropping any `.md` file into `~/.radhe/packs/`.

### Create a Custom Pack
```bash
radhe --create-pack
```
Follow the interactive prompts to set topics, key facts, and quiz style. The pack is saved to `~/.radhe/packs/` and immediately available via `radhe --pack <name>`.

## Language Mode 🌐
Radhe supports English, Hindi, and Hinglish responses:

| Language | Flag | Description |
|----------|------|-------------|
| English | `--lang en` | Default, clear English |
| Hindi | `--lang hi` | Devanagari script Hindi |
| Hinglish | `--lang hinglish` | Casual mix of Hindi + English in Roman script |

Set your default language permanently:
```bash
radhe --set-lang hinglish
```

Or use it per-command:
```bash
radhe --explain "binary search" --lang hi
radhe --chat --lang hinglish
```

## Study Modes & Quiz Difficulty 🎯

### Study Modes (`--mode`)
Tailor Radhe's explanation style to your study goal:
- **`normal`** (Default): In-depth conceptual explanations.
- **`exam`**: Ultra-short, direct, zero-fluff definitions and answers (max 2-3 sentences).
- **`revision`**: Bullet-style concise memory aids and quick revision facts.

```bash
# Get quick exam-style explanation
radhe --explain "polymorphism" --mode exam

# Generate revision notes
radhe --notes "quick sort" --mode revision
```

Set your default study mode permanently:
```bash
radhe --set-mode exam
```

### Quiz Difficulty (`--difficulty`)
Control the difficulty of generated interactive quizzes:
- **`easy`**: Factual recall and basic concepts.
- **`medium`** (Default): Intermediate conceptual understanding.
- **`hard`**: Advanced analytical questions, edge cases, and plausible distractors.

```bash
# Generate a hard MCQ quiz
radhe --quiz "recursion" --difficulty hard

# Generate an easy Q&A quiz from lecture notes
radhe --quiz-file notes.txt --difficulty easy
```

Set your default quiz difficulty permanently:
```bash
radhe --set-difficulty hard
```

## Requirements
- Windows 10/11
- ~1.5GB free disk space (for model)
- ~1GB RAM
- Internet connection only for install and updates

## Models
Powered by **Qwen2.5-Coder 1.5B Instruct** (Q4 quantized, runs on CPU)

## License
MIT
