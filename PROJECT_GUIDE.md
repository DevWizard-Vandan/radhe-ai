# Radhe AI — Project Guide & Onboarding

Welcome to Radhe AI! This guide is designed to help students, instructors, and users set up, configure, and use Radhe AI for offline study, quiz generation, and code review.

---

## 1. Quick Start

### Prerequisites
Before running Radhe AI, your environment requires two local components:
1. **Model**: Download the quantized Qwen2.5 model and save it to the home directory model folder:
   - **Windows Path**: `C:\Users\<username>\.radhe\models\Qwen2.5-Coder-1.5B-Instruct-Q4_K_M.gguf`
   - **Linux/macOS Path**: `~/.radhe/models/Qwen2.5-Coder-1.5B-Instruct-Q4_K_M.gguf`
2. **Inference Runner**: `llama-completion` (Linux/macOS) or `llama-completion.exe` (Windows) must be located in your system environment PATH.

### Initial Setup
Ensure directories are created by executing:
```bash
radhe init
```

Validate your setup using:
```bash
radhe doctor
```
If the doctor reports operational green checkmarks, your system is fully offline-ready!

---

## 2. Study Modes & Quiz Difficulty

Radhe AI features customized study models and quiz difficulty scales to tailor the assistant's behavior perfectly to your study targets.

### Study Modes (`--mode`)
Choose the style of explanations, notes, and responses:
- **`normal`** (Default): In-depth, beginner-friendly conceptual explanations.
- **`exam`**: Ultra-brief, direct, exam-style answers with zero fluff (max 2-3 sentences).
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
Control the challenge level of generated interactive exams:
- **`easy`**: Conceptual recall and literal comprehension from student notes.
- **`medium`** (Default): Standard conceptual understanding and basic applications.
- **`hard`**: Advanced analytical questions, tricky edge cases, and plausible distractors.

```bash
# Generate a hard MCQ quiz
radhe --quiz "recursion" --difficulty hard

# Generate an easy Q&A quiz from notes
radhe --quiz-file notes.txt --difficulty easy
```

Set your default quiz difficulty permanently:
```bash
radhe --set-difficulty hard
```

---

## 3. Offline Usage Statistics

Radhe AI runs 100% locally and values student privacy. The local statistics engine tracks commands atomically without ever saving query contents.

### Viewing Statistics
Print your offline study dashboard using:
```bash
radhe stats
```
This prints a clean console table of total commands run, individual feature usage, and subject pack usage sorted by popularity.

### Resetting Statistics
Wipe all usage history securely:
```bash
radhe stats --reset
```
Follow the interactive `[y/N]` confirmation prompt to safely clean the local stats data file.

---

## 4. Subject Pack Management

starter packs (`math`, `cs`, `science`) help tune Radhe's answers to specific courses.

### Using Subject Packs
```bash
radhe --pack cs
```

### Creating custom Packs
Launch the interactive pack wizard:
```bash
radhe --create-pack
```

### Deleting custom Packs
Interactive subject pack deletion:
```bash
radhe --delete-pack my_pack
```

---

## 5. Troubleshooting Local Setup Errors

### Error: `llama-completion not found. Run: radhe doctor`
* **Why this happens**: The CLI is looking for the execution runner binary in your environment PATH but cannot locate it.
* **Resolution**:
  1. Locate the folder where the `llama-completion` or `llama-completion.exe` binary resides.
  2. Add that folder path to your system `Path` variable.
  3. Restart your terminal session and verify with `radhe doctor`.

### Error: `MISSING: Model file ... not found in ~/.radhe/models/`
* **Why this happens**: The GGUF model was not downloaded or is not located in the correct directory.
* **Resolution**:
  1. Make sure your model is saved exactly under `~/.radhe/models/` (where `~` is your home directory).
  2. Run `radhe models` to list all GGUF files found in the models folder and verify their active status.
