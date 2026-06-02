# Project Status & Milestones

This document highlights the current developmental health, active milestones, and release stages of Radhe AI.

---

## Active Release Status

* **Current Stable Version**: `v0.7.1` (Local Analytics, Study Modes, Quiz Difficulty, Config Persistence, and Pack Deletion)
* **Status**: :white_check_mark: Active development / v0.7.1 successfully launched.

---

## Milestones Dashboard

### :checkered_flag: Milestone 0.7.1: Local Analytics & Student Experience (Completed)
* [x] **Study Modes (`--mode`)**: `exam` (ultra-short, direct, zero-fluff answers), `revision` (bullet-style memory aids), and `normal` (detailed conceptual explanations).
* [x] **Quiz Difficulty (`--difficulty`)**: Adapt MCQ and Q&A quiz generation dynamically to `easy`, `medium`, and `hard` settings.
* [x] **Local Analytics (`radhe stats`)**: Track `total_commands`, features, and alphabetical pack usage counters locally inside `stats.toml` preserving complete user privacy.
* [x] **Stats Reset (`--reset`)**: Add secure statistical data wipe after interactive `[y/N]` confirmation.
* [x] **Pack Deletion (`--delete-pack`)**: Implement interactive custom subject pack deletion from `~/.radhe/packs/`.
* [x] **Config Persistence (`--set-mode`/`--set-difficulty`)**: Allow users to save their preferred study modes and quiz difficulty defaults permanently to `config.toml`.
* [x] **Dynamic OS Binary Resolution**: Automatic runtime binary check (`llama-completion` vs `llama-completion.exe`) preventing platform crash errors.

### :checkered_flag: Milestone 0.7.0: Multilingual Hindi & Hinglish Support (Completed)
* [x] **Response Language Mode (`--lang hi|hinglish`)**: Multilingual output support using English, Devanagari script Hindi, or student-friendly casual Hinglish in Roman script.
* [x] **Language Defaults Persistence (`--set-lang`)**: Save language preference defaults permanently in `~/.radhe/config.toml`.
* [x] **Diagnostics Language Check**: Display active language selection in `radhe doctor` checks.

### :checkered_flag: Milestone 0.6.0: Subject Packs & Custom Pack Wizard (Completed)
* [x] **Subject Packs (`--pack <name>`)**: Tune Radhe's local model responses with customized starter subject packages (`math`, `cs`, `science`).
* [x] **Pack List (`--list-packs`)**: Scan local packs directories and display all installed packages in the terminal.
* [x] **Pack Creator Wizard (`--create-pack`)**: Interactive custom subject pack creator collecting topics, formulas, facts, and quiz styling.

### :checkered_flag: Milestone 0.5.0: Persistent Chat & Quiz From File (Completed)
* [x] **Persistent chat mode via `--chat`**: 6-turn rolling context window to preserve interactive dialogue history.
* [x] **`--quiz-file`**: Generate student quiz questions directly from text notes files.
* [x] **`--version` flag**: Print version and active GGUF model information instantly.
* [x] **Enhanced doctor**: Prints platform diagnostics, active model verification, and colored output indicators.

---

## Technical Health Metrics

* **Code Coverage**: Cargo unit and integration testing covers prompt compilations, CLI parsing, and file checking.
* **Binary Size**: Optimized at `< 1.5MB` via compilation profiling (`lto = true`, `strip = true`).
* **Memory Usage**: Under `20MB` runtime footprint (excluding the GGUF model weights in the inference backend).
