# Feature List & Prompts Reference

This document provides a detailed catalog of the features, CLI flags, prompt engineering templates, token bounds, and output formatting rules implemented in Radhe AI.

---

## 1. Code Generation (`--code`)
Designed to write pure, compilable source code for computer science assignments, bypassing conversational explanations.

* **CLI Argument**: `radhe --code "<task>"`
* **Token Limit**: Exactly `300` tokens.
* **Underlying Prompt Engineering**:
  ```text
  You are a coding assistant. Return ONLY valid compilable code with zero explanation. No markdown, no backticks, no comments. Just raw code.
  Task: {task}
  ```
  *(Note: If a specific programming language keyword is detected, e.g. "C", "Rust", "Python", the suffix `, respect the exact language specified.` is automatically appended to the task string.)*
* **Output Processing**: Filters out markdown blocks (```), strips compiler preambles, and truncates on stop keywords (e.g. `Explanation:`).

---

## 2. Concept Explanation (`--explain`)
Generates structural, beginner-friendly conceptual definitions for programming topics.

* **CLI Argument**: `radhe --explain "<topic>"`
* **Token Limit**: Exactly `200` tokens.
* **Underlying Prompt Engineering**:
  ```text
  Explain '{topic}' in exactly 5 bullet points for a beginner programmer. Each bullet must be one sentence. Stop after 5 bullets. Do not repeat yourself.

  Explanation:
  ```
* **Output Processing**: Echo is stripped using the `### RESPONSE:` boundary.

---

## 3. Academic Revision Notes (`--notes`)
Creates extremely concise bullet facts suitable for exam preparation and quick student reference.

* **CLI Argument**: `radhe --notes "<topic>"`
* **Token Limit**: Exactly `150` tokens.
* **Underlying Prompt Engineering**:
  ```text
  Write exactly 6 short student notes about '{topic}'. Format: bullet points. Each bullet = one fact. Max 10 words per bullet. Stop after 6 bullets. No repetition.

  Notes:
  ```

---

## 4. Automatic Bug Fixer (`--fix`)
Syntactically analyzes source files, strips clues, detects the language automatically, and outputs corrected versions.

* **CLI Argument**: `radhe --fix "<filepath>"`
* **Token Limit**: Exactly `400` tokens.
* **Language Detection (Case-insensitive)**:
  - `.c` -> `C`
  - `.cpp` -> `C++`
  - `.py` -> `Python`
  - `.rs` -> `Rust`
  - `.java` -> `Java`
  - Defaults to `code`
* **Underlying Prompt Engineering**:
  ```text
  You are a C/{language} compiler and debugger. The following code has syntax errors and logic bugs. Rewrite it completely with ALL bugs fixed. Output ONLY the fixed code. No explanations, no comments about what was fixed, no markdown fences.
  BROKEN CODE:
  {cleaned_file_content}
  FIXED CODE:
  ```
* **Interactive Hint Stripping**: Pre-inference filtering automatically removes any lines containing `// bug:` or `# bug:` (case-insensitive) to prevent spoiling compiler tests.

---

## 5. Diagnostics & Environment (`doctor`)
Performs static system analysis to ensure dependencies are fully functional.

* **Subcommand**: `radhe doctor`
* **Checks Conducted**:
  1. Searches for `llama-completion.exe` in the host's system PATH.
  2. Resolves the user's home directory and checks for the existence of `qwen2.gguf` under `~/.radhe/models/`.

---

## 6. Directory Initialization (`init`)
Creates the standard workspace folders.

* **Subcommand**: `radhe init`
* **Folders Created**:
  - `~/.radhe/models/` (Local model repository)
  - `~/installer/` (System scripts)

---

## 7. Interactive REPL Mode
Launches a beautiful, state-of-the-art terminal wrapper when the CLI is run with no arguments.

* **Trigger**: `radhe`
* **Colors (Via `colored` crate)**:
  - Cyan: Main welcome headers.
  - Yellow: Shortcut hints and guidelines.
  - Green (Bold): Prompt marker `>>> `.
* **Commands**:
  - `/exit`: Terminate session.
  - `/clear`: Refreshes screen by printing 50 newlines.
  - `--code <text>`: Triggers code generation prompt.
  - `--explain <text>`: Triggers explanation prompt.
  - `--notes <text>`: Triggers notes prompt.
  - Any raw input: Evaluates as a general prompt in `--explain` mode.
