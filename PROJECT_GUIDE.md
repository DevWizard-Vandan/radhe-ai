# Radhe AI - Project Guide & Onboarding

Welcome to Radhe AI! This guide is designed to help students, instructors, and users set up, configure, and use Radhe AI for offline study and assignment review.

---

## 1. Quick Start

### Prerequisites
Before running Radhe AI, your environment requires two local dependencies:
1. **Model**: Download the quantized Qwen2 model (`qwen2.gguf`) and save it to the home directory model folder:
   - **Windows Path**: `C:\Users\<username>\.radhe\models\qwen2.gguf`
   - **Linux/macOS Path**: `~/.radhe/models/qwen2.gguf`
2. **Inference Runner**: `llama-completion.exe` must be located in your system environment PATH.

### Initial Setup
Ensure directories are created by executing:
```bash
radhe init
```

Validate your setup using:
```bash
radhe doctor
```
If the doctor reports `OK: llama-completion.exe found` and `OK: model found`, your environment is fully offline-ready!

---

## 2. Common Student Workflows

### Scenario A: Programming Assignment Help
When you need to construct a specific function (e.g. bubble sort in C) but have zero internet access:
```bash
radhe --code "bubble sort in C"
```
The output will strictly contain compilable C code that you can pipe directly to a compiler.

### Scenario B: Exam Revision Sheets
To review complex operating systems or compiler topics before an exam, generate ultra-brief facts:
```bash
radhe --notes "Page replacement algorithms"
```

### Scenario C: Debugging Broken Code
If you are receiving syntax errors on your homework file (`assignment.py`):
```bash
radhe --fix assignment.py
```
This reads the python file, strips out bug markers, repairs syntax/logic issues, and dumps the pure fixed script.

---

## 3. Keyboard & Control inside the REPL

Launch the interactive loop by typing `radhe` with no flags:
```bash
radhe
```

* **Keyboard Shortcuts**:
  - `Ctrl+C`: Instantly terminates the session with a friendly exit screen.
  - `/exit`: Exits the loop.
  - `/clear`: Clears current terminal buffer.
* **Inline Prompts**: You can switch modes on-the-fly inside the loop:
  - Prefix with `--code` to generate pure code.
  - Prefix with `--notes` to write bullet notes.

---

## 4. Troubleshooting Local Setup Errors

### Error: `llama-completion not found. Run: radhe doctor`
* **Why this happens**: The CLI is looking for `llama-completion.exe` in your environment PATH but cannot locate it.
* **Resolution**:
  1. Open your system Environment Variables.
  2. Add the path to the folder containing `llama-completion.exe` into your `Path` variable.
  3. Restart your terminal session and verify with `radhe doctor`.

### Error: `MISSING: download model to ~/.radhe/models/qwen2.gguf`
* **Why this happens**: The qwen2 model was not saved with the correct file name or in the correct folder.
* **Resolution**:
  1. Go to your home directory (`C:\Users\<username>` on Windows).
  2. Create a folder named `.radhe` and a subfolder named `models` if they don't exist.
  3. Ensure the downloaded model is named exactly `qwen2.gguf`.
