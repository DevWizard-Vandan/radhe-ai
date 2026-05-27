# Changelog
## [v0.5.0] — 2026-05-26
- feat: `--quiz-file <file>` — generate quiz questions from student notes
- feat: `--version` — print version and active model instantly
- feat: enhanced `radhe doctor` — version header, active model validation, colored output
- feat: `RADHE_DEBUG=1` env debug mode — logs prompts, paths, and raw output to stderr
- fix: standardized all error messages to `Error/Hint` format with colored output
- fix: strip UNC prefix from canonicalized paths on Windows
- test: added 7 CLI smoke and unit tests
## [v0.4.0] — 2026-05-26
- feat: `--chat` persistent conversation mode with 6-turn rolling context window
- feat: ChatML prompt format for Qwen instruct models
- fix: context history passing across chat turns
## [v0.3.0] — 2026-05-26
- feat: `--summarize <file>` — summarize notes into 5 bullets
- feat: `radhe update` — self-update to latest release
- feat: upgraded default model to Qwen2.5-Coder 1.5B Instruct (Q4)
- feat: `--quiz` interactive MCQ engine with score tracking
- feat: `config.toml` configuration system
- feat: `radhe models` — list installed GGUF models
## [v0.1.0] — 2026-05-25
- feat: core CLI with `--code`, `--explain`, `--notes`, `--fix`
- feat: interactive REPL mode
- feat: `radhe doctor` environment diagnostics
- feat: `radhe init` directory setup
- feat: Windows PowerShell installer (`install.ps1`)
- feat: Linux bash installer (`install.sh`)
