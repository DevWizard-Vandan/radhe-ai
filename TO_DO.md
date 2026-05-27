# Radhe AI — TO DO

> Last updated: v0.5.0 (2026-05-26)

---

## 🔴 Do Tomorrow (High Priority)

### Docs & Status Alignment
- [x] Update `PROJECT_STATUS.md` — set current stable to `v0.5.0`, add v0.4.0 and v0.5.0 milestone entries, mark Windows installer tasks as complete
- [x] Update `FEATURE_LIST.md` — fix config example (`qwen2.gguf` → `Qwen2.5-Coder-1.5B-Instruct-Q4_K_M.gguf`), add `--quiz-file` section, update `--chat` to show ChatML prompt format
- [x] Check README features table includes `--quiz-file` row with example call

### Quick CLI Improvements
- [x] Add `radhe --version` — print `Radhe AI vX.Y.Z` + active model name using `env!("CARGO_PKG_VERSION")`
- [x] Enhance `radhe doctor` — check for `Qwen2.5-Coder-1.5B-Instruct-Q4_K_M.gguf` instead of `qwen2.gguf`, confirm binary version, validate llama.cpp health

### Seed Tests
- [x] Add `tests/cli_smoke.rs` — test CLI parsing for `--summarize`, `--quiz-file`, `--chat` without inference
- [x] Add unit tests for prompt builder functions (verify "exactly 5 questions", "Q1/A1 format" strings are present)

---

## 🟡 Short-Term (Next 1–2 Weeks)

### Cross-Platform Install
- [x] Write `install.sh` for Linux — mirrors `install.ps1`: create `~/.radhe/{bin,models}`, download `radhe` Linux binary, download llama.cpp Linux CPU build, download Qwen 1.5B model, add to PATH
- [x] Add "Linux Install" section to README with curl|bash one-liner
- [ ] Decide macOS strategy — full `install.sh` or document manual steps only
- [ ] Update README with "Manual install (advanced)" section linking to GitHub release assets

### Quality & Reliability
- [ ] Add `radhe update` test using mocked GitHub response — factor version comparison into pure function
- [ ] Add regression test for ChatML prompt + stop-string logic
- [x] Audit error messages for `--summarize`, `--quiz-file`, `--fix`, `update` — make them consistent and friendly
- [x] Add `RADHE_DEBUG=1` env flag to print prompts, paths, and llama.cpp commands without affecting normal users

### Docs Hygiene
- [ ] Backfill CHANGELOG with proper entries for v0.4.0 and v0.5.0
- [ ] Add a release checklist comment to `PROJECT_STATUS.md` so docs stay in sync with future releases

---

## 🟢 Medium-Term (Next Month)

### Editor Integrations
- [ ] VS Code extension MVP — config UI to point at `radhe` binary; commands: "Explain selection", "Fix selection", "Generate quiz from file"
- [ ] Vim/Neovim Lua plugin — pipe visual selection into `radhe --fix` or `--explain`, replace with output

### Language & Content Expansion
- [ ] Hindi/Hinglish prompt templates for `--explain` and `--notes` (Hinglish explanations, English keywords)
- [ ] Add `--lang` flag (`--lang hi` / `--lang en-hi`) to toggle bilingual mode
- [ ] Subject packs — specialized prompt modes for DSA, OS, DBMS, CN, OOPS

---

## 🔵 Long-Term Goals

- [ ] GPU mode — detect local GPU, use appropriate llama.cpp build, surface hint in `radhe doctor`
- [ ] Windows packaging — Scoop or winget manifest for `radhe`
- [ ] Linux packaging — .deb package or Homebrew tap
- [ ] Flashcards mode — `radhe --flashcards <file>` generates Q&A flashcard sets from notes
- [ ] REPL polish — persistent chat history saved to disk, `/history`, `/clear` improvements

---

## ⚠️ Risks & Blockers

| Risk | Mitigation |
|---|---|
| `llama-completion.exe` asset naming changes in llama.cpp releases and breaks installer | Pin a specific llama.cpp version in installer; bump manually; add `doctor` runtime check |
| Linux/macOS users have no install story yet | Fast-follow with `install.sh` and README Linux section |
| Docs drift behind code (already happened once at v0.5.0) | After every release: update PROJECT_STATUS + FEATURE_LIST + README + CHANGELOG as a checklist |
| No tests = silent regressions (ChatML, path canonicalization, update) | Start with fast prompt-builder unit tests; keep inference out of test suite |
