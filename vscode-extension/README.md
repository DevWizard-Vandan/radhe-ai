# Radhe AI — VS Code Extension
Offline AI assistant for students, powered by local Qwen model via the `radhe` CLI.
## Commands
| Command | Shortcut | What it does |
|---|---|---|
| Radhe: Explain Selection | Right-click → Radhe | Explains selected text in a side panel |
| Radhe: Fix Selection | Right-click → Radhe | Fixes selected code in-place |
| Radhe: Generate Quiz from File | Right-click → Radhe | Generates quiz from current file |
## Requirements
- Radhe AI installed: `irm https://raw.githubusercontent.com/DevWizard-Vandan/radhe-ai/main/install.ps1 | iex`
- VS Code 1.85+
## Settings
| Setting | Default | Description |
|---|---|---|
| `radhe.binaryPath` | `radhe` | Path to radhe binary if not in PATH |
