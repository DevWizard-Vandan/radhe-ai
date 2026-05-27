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

## Requirements
- Windows 10/11
- ~1.5GB free disk space (for model)
- ~1GB RAM
- Internet connection only for install and updates

## Models
Powered by **Qwen2.5-Coder 1.5B Instruct** (Q4 quantized, runs on CPU)

## License
MIT
