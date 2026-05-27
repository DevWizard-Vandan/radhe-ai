# How to install Radhe AI extension locally
## Option A — Install from .vsix (recommended)
1. Install vsce: `npm install -g @vscode/vsce`
2. In this folder: `vsce package`
3. This creates `radhe-ai-0.1.0.vsix`
4. In VS Code: `Ctrl+Shift+P` → `Extensions: Install from VSIX` → select the file
## Option B — Install directly from folder
1. Copy this `vscode-extension/` folder to `~/.vscode/extensions/radhe-ai-0.1.0/`
2. Restart VS Code
## Usage
- Select text → Right-click → **Radhe: Explain Selection**
- Select code → Right-click → **Radhe: Fix Selection**
- Open a notes file → Right-click → **Radhe: Generate Quiz from File**
- Or use keyboard shortcuts: `Ctrl+Shift+E`, `Ctrl+Shift+F`, `Ctrl+Shift+Q`
