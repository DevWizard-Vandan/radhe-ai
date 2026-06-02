use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use serde::Deserialize;
use std::{
    fs,
    io::ErrorKind,
    path::PathBuf,
    process::{Command, Stdio},
};

fn debug_log(msg: &str) {
    if std::env::var("RADHE_DEBUG").unwrap_or_default() == "1" {
        eprintln!("[DEBUG] {}", msg);
    }
}

fn clean_path(path: &std::path::Path) -> String {
    let s = path.to_string_lossy().into_owned();
    if s.starts_with("\\\\?\\") {
        s[4..].to_string()
    } else {
        s
    }
}

fn lang_system_prompt(lang: &str) -> &'static str {
    match lang {
        "hi" => "You are Radhe, an AI assistant. Always respond in simple Hindi (Devanagari script). Use clear, student-friendly language. Avoid complex Sanskrit terms — prefer everyday Hindi words.",
        "hinglish" => "You are Radhe, an AI assistant. Always respond in Hinglish — a natural mix of Hindi and English commonly used by Indian students. Write Hindi words in Roman script (not Devanagari). Keep it casual and friendly, like explaining to a classmate.",
        _ => "You are Radhe, a helpful AI assistant. Respond in clear, simple English.",
    }
}

fn find_pack(name: &str) -> Option<PathBuf> {
    let filename = format!("{}.md", name);

    // 1. Same directory as the current binary
    if let Ok(current_exe) = std::env::current_exe() {
        if let Some(parent) = current_exe.parent() {
            let path = parent.join("packs").join(&filename);
            if path.exists() {
                return Some(path);
            }
        }
    }

    // 2. Relative to current working directory
    let cwd_path = PathBuf::from("packs").join(&filename);
    if cwd_path.exists() {
        return Some(cwd_path);
    }

    // 3. ~/.radhe/packs/
    if let Some(home) = dirs::home_dir() {
        let path = home.join(".radhe").join("packs").join(&filename);
        if path.exists() {
            return Some(path);
        }
    }

    None
}

fn run_create_pack() {
    use std::io::{BufRead, Write};
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    writeln!(out, "\n\x1b[36m[Radhe] Custom Pack Creator\x1b[0m").unwrap();
    writeln!(out, "────────────────────────────────────────").unwrap();
    // Pack name
    write!(out, "Pack name (e.g. history, economics): ").unwrap();
    out.flush().unwrap();
    let mut name = String::new();
    stdin.lock().read_line(&mut name).unwrap();
    let name = name.trim().to_lowercase().replace(' ', "_");
    if name.is_empty() { eprintln!("Error: Pack name cannot be empty."); return; }
    // Sanitize: only allow alphanumeric, underscore, hyphen — no path separators
    if name.chars().any(|c| !c.is_alphanumeric() && c != '_' && c != '-') {
        eprintln!("{}: Pack name '{}' contains invalid characters.", "Error".red(), name);
        eprintln!("{}: Use only letters, numbers, underscores, and hyphens.", "Hint".yellow());
        return;
    }
    // Display name
    write!(out, "Display name (e.g. History, Economics): ").unwrap();
    out.flush().unwrap();
    let mut display = String::new();
    stdin.lock().read_line(&mut display).unwrap();
    let display = display.trim().to_string();
    // Topics
    writeln!(out, "Enter topics (one per line, blank line to finish):").unwrap();
    let mut topics: Vec<String> = Vec::new();
    loop {
        write!(out, "  Topic: ").unwrap();
        out.flush().unwrap();
        let mut t = String::new();
        stdin.lock().read_line(&mut t).unwrap();
        let t = t.trim().to_string();
        if t.is_empty() { break; }
        topics.push(t);
    }
    // Key formulas / facts
    writeln!(out, "Enter key facts or formulas (one per line, blank line to finish):").unwrap();
    let mut facts: Vec<String> = Vec::new();
    loop {
        write!(out, "  Fact: ").unwrap();
        out.flush().unwrap();
        let mut f = String::new();
        stdin.lock().read_line(&mut f).unwrap();
        let f = f.trim().to_string();
        if f.is_empty() { break; }
        facts.push(f);
    }
    // Quiz style
    write!(out, "Quiz style (e.g. MCQ, short answer, NCERT-style): ").unwrap();
    out.flush().unwrap();
    let mut quiz = String::new();
    stdin.lock().read_line(&mut quiz).unwrap();
    let quiz = quiz.trim().to_string();
    // Build markdown
    let mut md = format!("# {} Pack — Radhe AI\n\n", display);
    md.push_str("## Topics\n");
    for t in &topics { md.push_str(&format!("- {}\n", t)); }
    if !facts.is_empty() {
        md.push_str("\n## Key Facts / Formulas\n");
        for f in &facts { md.push_str(&format!("- {}\n", f)); }
    }
    if !quiz.is_empty() {
        md.push_str(&format!("\n## Quiz Style\n{}\n", quiz));
    }
    // Save to ~/.radhe/packs/<name>.md
    let pack_dir = dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join(".radhe")
        .join("packs");
    std::fs::create_dir_all(&pack_dir).ok();
    let pack_path = pack_dir.join(format!("{}.md", name));
    if let Err(e) = std::fs::write(&pack_path, &md) {
        eprintln!("{}: Could not save pack file '{}': {}", "Error".red(), pack_path.display(), e);
        eprintln!("{}: Check permissions for ~/.radhe/packs/ and try again.", "Hint".yellow());
        return;
    }
    writeln!(out, "\n\x1b[32m[Radhe] Pack saved: {}\x1b[0m", pack_path.display()).unwrap();
    writeln!(out, "Use it with: radhe --pack {}", name).unwrap();
}

fn run_list_packs() -> Result<()> {
    let mut packs = std::collections::BTreeSet::new();

    // Check ./packs/
    if let Ok(entries) = fs::read_dir("packs") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    packs.insert(stem.to_string());
                }
            }
        }
    }

    // Check ~/.radhe/packs/
    if let Some(home) = dirs::home_dir() {
        let home_packs_dir = home.join(".radhe").join("packs");
        if let Ok(entries) = fs::read_dir(home_packs_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        packs.insert(stem.to_string());
                    }
                }
            }
        }
    }

    // Check same dir as binary
    if let Ok(current_exe) = std::env::current_exe() {
        if let Some(parent) = current_exe.parent() {
            let exe_packs_dir = parent.join("packs");
            if let Ok(entries) = fs::read_dir(exe_packs_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                            packs.insert(stem.to_string());
                        }
                    }
                }
            }
        }
    }

    if packs.is_empty() {
        println!("No subject packs found.");
    } else {
        println!("Available subject packs:");
        for pack in packs {
            println!("  - {}", pack);
        }
    }
    Ok(())
}

#[derive(Deserialize, Default, Debug)]
struct RadheConfig {
    model: Option<String>,
    max_tokens: Option<u32>,
    lang: Option<String>,
}

#[derive(Parser, Debug)]
#[command(name = "radhe")]
#[command(about = "Tiny offline terminal AI assistant for students")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(long)]
    version: bool,

    #[arg(value_name = "PROMPT")]
    prompt: Option<String>,

    #[arg(long, value_name = "TASK")]
    code: Option<String>,

    #[arg(long, value_name = "TOPIC")]
    explain: Option<String>,

    #[arg(long, value_name = "TOPIC")]
    notes: Option<String>,

    #[arg(long, value_name = "FILE")]
    fix: Option<String>,

    #[arg(long, value_name = "FILE")]
    summarize: Option<String>,

    #[arg(long)]
    chat: bool,

    #[arg(long, value_name = "TOPIC")]
    quiz: Option<String>,

    #[arg(long, value_name = "FILE")]
    quiz_file: Option<String>,

    #[arg(long, value_name = "COUNT")]
    count: Option<u8>,

    #[arg(long, value_name = "FILENAME")]
    model: Option<String>,

    #[arg(long, value_name = "MAX_TOKENS")]
    max_tokens: Option<u32>,

    #[arg(long, value_name = "NAME")]
    pack: Option<String>,

    #[arg(long)]
    list_packs: bool,

    /// Launch interactive wizard to create a custom subject pack
    #[arg(long = "create-pack")]
    create_pack: bool,

    /// Delete an installed subject pack
    #[arg(long = "delete-pack", value_name = "NAME")]
    delete_pack: Option<String>,

    /// Response language: 'en' (default), 'hi' (Hindi), 'hinglish' (mixed Hindi+English)
    #[arg(long = "lang", value_name = "LANG")]
    lang: Option<String>,

    /// Set default language in config: 'en', 'hi', or 'hinglish'
    #[arg(long = "set-lang", value_name = "LANG")]
    set_lang: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init,
    Doctor,
    Models,
    Update,
}

fn main() -> Result<()> {
    let config_dir = dirs::home_dir()
        .context("unable to find user home directory")?
        .join(".radhe");
    
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    let config_path = config_dir.join("config.toml");
    if !config_path.exists() {
        let default_config = r#"# Radhe AI Configuration
# Change model to use a different GGUF file from ~/.radhe/models/
model = "Qwen2.5-Coder-1.5B-Instruct-Q4_K_M.gguf"
max_tokens = 300
"#;
        fs::write(&config_path, default_config)?;
    }

    let config: RadheConfig = if config_path.exists() {
        let content = fs::read_to_string(&config_path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    } else {
        RadheConfig::default()
    };

    let mut cli = Cli::parse();

    if cli.version {
        let version = env!("CARGO_PKG_VERSION");
        let model = cli.model
            .clone()
            .or_else(|| config.model.clone())
            .unwrap_or_else(|| "Qwen2.5-Coder-1.5B-Instruct-Q4_K_M.gguf".to_string());
        println!("Radhe AI v{}", version);
        println!("Model: {}", model);
        std::process::exit(0);
    }

    // Handle --set-lang early
    if let Some(ref new_lang) = cli.set_lang {
        let valid = ["en", "hi", "hinglish"];
        if !valid.contains(&new_lang.as_str()) {
            eprintln!("{}: Invalid language '{}'. Valid options: en, hi, hinglish", "Error".red(), new_lang);
            eprintln!("{}: Use --set-lang en, --set-lang hi, or --set-lang hinglish", "Hint".yellow());
            std::process::exit(1);
        }
        // Read existing config and update lang line
        let content = fs::read_to_string(&config_path).unwrap_or_default();
        let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
        let mut found_lang = false;
        for line in lines.iter_mut() {
            if line.starts_with("lang") && line.contains('=') {
                *line = format!("lang = \"{}\"", new_lang);
                found_lang = true;
                break;
            }
        }
        if !found_lang {
            lines.push(format!("lang = \"{}\"", new_lang));
        }
        fs::write(&config_path, lines.join("\n") + "\n")?;
        let label = match new_lang.as_str() {
            "hi" => "Hindi",
            "hinglish" => "Hinglish",
            _ => "English",
        };
        println!("Language set to {} ({}). All future responses will use this language.", label, new_lang);
        std::process::exit(0);
    }

    if cli.list_packs {
        run_list_packs()?;
        std::process::exit(0);
    }

    if cli.create_pack {
        run_create_pack();
        return Ok(());
    }

    if let Some(ref pack_name) = cli.delete_pack {
        // Sanitize name
        if pack_name.chars().any(|c| !c.is_alphanumeric() && c != '_' && c != '-') {
            eprintln!("{}: Invalid pack name '{}'.", "Error".red(), pack_name);
            eprintln!("{}: Use only letters, numbers, underscores, and hyphens.", "Hint".yellow());
            return Ok(());
        }
        let pack_path = find_pack(pack_name);
        match pack_path {
            None => {
                eprintln!("{}: Pack '{}' not found.", "Error".red(), pack_name);
                eprintln!("{}: Run 'radhe --list-packs' to see installed packs.", "Hint".yellow());
            }
            Some(path) => {
                use std::io::{self, Write};
                print!("Delete pack '{}' at {}? [y/N]: ", pack_name, path.display());
                io::stdout().flush().ok();
                let mut confirm = String::new();
                io::stdin().read_line(&mut confirm).ok();
                if confirm.trim().to_lowercase() == "y" {
                    fs::remove_file(&path)?;
                    println!("{}: Pack '{}' deleted.", "[Radhe]".green(), pack_name);
                } else {
                    println!("Aborted.");
                }
            }
        }
        return Ok(());
    }

    if let Some(ref path) = cli.summarize {
        let abs_path = std::fs::canonicalize(path)
            .unwrap_or_else(|_| std::path::PathBuf::from(path));
        let resolved = clean_path(&abs_path);
        debug_log(&format!("Resolved path: {}", resolved));
        cli.summarize = Some(resolved);
    }

    if let Some(ref path) = cli.fix {
        let abs_path = std::fs::canonicalize(path)
            .unwrap_or_else(|_| std::path::PathBuf::from(path));
        cli.fix = Some(clean_path(&abs_path));
    }

    if let Some(ref path) = cli.quiz_file {
        let abs_path = std::fs::canonicalize(path)
            .unwrap_or_else(|_| std::path::PathBuf::from(path));
        let resolved = clean_path(&abs_path);
        debug_log(&format!("Resolved path: {}", resolved));
        cli.quiz_file = Some(resolved);
    }

    // Keep 0.5B accessible via --model qwen-0.5b override flag
    let active_model = cli.model
        .clone()
        .or_else(|| config.model.clone())
        .unwrap_or_else(|| "Qwen2.5-Coder-1.5B-Instruct-Q4_K_M.gguf".to_string());

    let active_max_tokens = cli.max_tokens
        .or(config.max_tokens)
        .unwrap_or(300);

    // Resolve language: CLI flag > config.toml > default "en"
    let active_lang = cli.lang
        .clone()
        .or_else(|| config.lang.clone())
        .unwrap_or_else(|| "en".to_string());

    debug_log(&format!("Config loaded — model: {}, max_tokens: {}, lang: {}", active_model, active_max_tokens, active_lang));

    if let Some(ref pack_name) = cli.pack {
        let pack_path = find_pack(pack_name);
        if pack_path.is_none() {
            eprintln!("{}: Pack file '{}.md' not found.", "Error".red(), pack_name);
            eprintln!("{}: Run 'radhe --list-packs' to see available packs.", "Hint".yellow());
            std::process::exit(1);
        }
        let pack_path = pack_path.unwrap();
        let pack_content = match fs::read_to_string(&pack_path) {
            Ok(content) => content,
            Err(_) => {
                eprintln!("{}: Could not read pack file '{}'", "Error".red(), pack_path.display());
                eprintln!("{}: Check file permissions and try again.", "Hint".yellow());
                std::process::exit(1);
            }
        };

        let formatted_name = if pack_name.to_lowercase() == "cs" {
            "CS".to_string()
        } else {
            let mut chars = pack_name.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
            }
        };

        println!("[Radhe] Loaded {} Pack. Ask your question:", formatted_name);

        use std::io;
        let mut question = String::new();
        if io::stdin().read_line(&mut question).is_err() {
            eprintln!("{}: Failed to read question from stdin.", "Error".red());
            std::process::exit(1);
        }
        let question = question.trim();
        if question.is_empty() {
            println!("No question asked. Exiting.");
            std::process::exit(0);
        }

        let lang_prefix = if active_lang != "en" {
            format!("\n\n{}", lang_system_prompt(&active_lang))
        } else {
            String::new()
        };
        let prompt = format!("<|im_start|>system\n{}{}\n<|im_end|>\n<|im_start|>user\n{}\n<|im_end|>\n<|im_start|>assistant\n", pack_content, lang_prefix, question);
        let max_tokens = active_max_tokens.max(500);

        let output = run_inference(&prompt, &active_model, max_tokens, "chat")
            .context("failed to run local inference")?;
        println!("{}", output);
        return Ok(());
    }

    match cli.command {
        Some(Commands::Init) => {
            println!("Initializing Radhe AI directories...");
            init_dirs()?;
            println!("Done.");
            return Ok(());
        }
        Some(Commands::Doctor) => {
            run_doctor(&active_model, &active_lang);
            return Ok(());
        }
        Some(Commands::Models) => {
            run_models(&active_model)?;
            return Ok(());
        }
        Some(Commands::Update) => {
            run_update()?;
            return Ok(());
        }
        None => {}
    }

    let is_repl = cli.prompt.is_none()
        && cli.code.is_none()
        && cli.explain.is_none()
        && cli.notes.is_none()
        && cli.fix.is_none()
        && cli.quiz.is_none()
        && cli.summarize.is_none()
        && cli.quiz_file.is_none()
        && !cli.chat;

    if is_repl {
        run_repl(&active_model, &active_lang)?;
        return Ok(());
    }

    let mode = if cli.code.is_some() {
        "code"
    } else if cli.explain.is_some() {
        "explain"
    } else if cli.notes.is_some() {
        "notes"
    } else if cli.fix.is_some() {
        "fix"
    } else if cli.quiz.is_some() {
        "quiz"
    } else if cli.summarize.is_some() {
        "summarize"
    } else if cli.quiz_file.is_some() {
        "quiz_file"
    } else if cli.chat {
        "chat"
    } else {
        "prompt"
    };

    if mode == "chat" {
        run_chat(&active_model, &active_lang)?;
        return Ok(());
    }

    let prompt = build_prompt(&cli, &active_lang)?;
    let max_tokens = match mode {
        "code" => 300,
        "explain" => 200,
        "notes" => 150,
        "fix" => 400,
        "quiz" => 500,
        "summarize" => 200,
        "quiz_file" => 400,
        _ => active_max_tokens,
    };

    if mode == "summarize" {
        if let Some(file_path_str) = &cli.summarize {
            let path = PathBuf::from(file_path_str);
            if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                println!("Summarizing {}...", filename);
            } else {
                println!("Summarizing {}...", file_path_str);
            }
        }
    }

    if mode == "quiz_file" {
        if let Some(file_path_str) = &cli.quiz_file {
            let path = PathBuf::from(file_path_str);
            if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                println!("Generating quiz from {}...", filename);
            } else {
                println!("Generating quiz from {}...", file_path_str);
            }
        }
    }

    let output = run_inference(&prompt, &active_model, max_tokens, mode)
        .context("failed to run local inference")?;

    if mode == "quiz" {
        run_quiz(&output);
    } else {
        println!("{output}");
    }
    Ok(())
}

fn build_prompt(cli: &Cli, lang: &str) -> Result<String> {
    let lang_suffix = if lang != "en" {
        format!("\n\n{}", lang_system_prompt(lang))
    } else {
        String::new()
    };
    if let Some(task) = &cli.code {
        let has_lang_hint = task
            .to_lowercase()
            .split(|c: char| !c.is_alphanumeric() && c != '+' && c != '#')
            .any(|word| {
                matches!(
                    word,
                    "c" | "c++"
                        | "cpp"
                        | "c#"
                        | "csharp"
                        | "rust"
                        | "python"
                        | "java"
                        | "javascript"
                        | "js"
                        | "typescript"
                        | "ts"
                        | "go"
                        | "golang"
                        | "ruby"
                        | "php"
                        | "swift"
                        | "kotlin"
                        | "bash"
                        | "shell"
                        | "powershell"
                        | "sql"
                        | "html"
                        | "css"
                        | "assembly"
                )
            });

        let mut task_str = task.clone();
        if has_lang_hint {
            task_str.push_str(", respect the exact language specified.");
        }

        return Ok(format!(
            "You are a coding assistant. Return ONLY valid compilable code with zero explanation. No markdown, no backticks, no comments. Just raw code.{}
Task: {task_str}", lang_suffix
        ));
    }

    if let Some(topic) = &cli.explain {
        return Ok(format!(
            "Explain '{topic}' in exactly 5 bullet points for a beginner programmer. Each bullet must be one sentence. Stop after 5 bullets. Do not repeat yourself.{}\n\nExplanation:", lang_suffix
        ));
    }

    if let Some(topic) = &cli.notes {
        return Ok(format!(
            "Give exactly 6 bullet points about '{topic}' for a student. Format strictly as:
- [fact 1]
- [fact 2]
- [fact 3]
- [fact 4]
- [fact 5]
- [fact 6]
Each bullet = one unique fact. Max 15 words per bullet. Start directly with the first bullet, no intro paragraph.{}", lang_suffix
        ));
    }

    if let Some(file_path_str) = &cli.fix {
        let path = PathBuf::from(file_path_str);
        if !path.exists() {
            eprintln!("{}: Could not read file '{}'", "Error".red(), file_path_str);
            eprintln!("{}: Check the file path and try again.", "Hint".yellow());
            std::process::exit(1);
        }
        let code = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("{}: Could not read file '{}'", "Error".red(), file_path_str);
                eprintln!("{}: Check the file path and try again.", "Hint".yellow());
                std::process::exit(1);
            }
        };
        let trimmed_code = code.trim();
        if trimmed_code.is_empty() {
            eprintln!("{}: File '{}' is empty.", "Error".red(), file_path_str);
            eprintln!("{}: Nothing to fix — add some code first.", "Hint".yellow());
            std::process::exit(1);
        }

        // Strip any line containing // bug: or # bug: before building the prompt
        let cleaned_code_lines: Vec<&str> = code
            .lines()
            .filter(|line| {
                let line_lower = line.to_lowercase();
                !line_lower.contains("// bug:") && !line_lower.contains("# bug:")
            })
            .collect();
        let cleaned_code = cleaned_code_lines.join("\n");

        let ext_lower = path.extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();

        let language = match ext_lower.as_str() {
            "c" => "C",
            "cpp" => "C++",
            "py" => "Python",
            "rs" => "Rust",
            "java" => "Java",
            _ => "code",
        };

        return Ok(format!(
            "You are a C/{} compiler and debugger. The following code has syntax errors and logic bugs. Rewrite it completely with ALL bugs fixed. Output ONLY the fixed code. No explanations, no comments about what was fixed, no markdown fences.
BROKEN CODE:
{}
FIXED CODE:\n",
            language, cleaned_code
        ));
    }
    // Note: --fix mode intentionally does NOT apply lang_suffix since output must be raw code.

    if let Some(file_path_str) = &cli.summarize {
        let path = PathBuf::from(file_path_str);
        if !path.exists() {
            eprintln!("{}: Could not read file '{}'", "Error".red(), file_path_str);
            eprintln!("{}: Check the file path and try again.", "Hint".yellow());
            std::process::exit(1);
        }
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("{}: Could not read file '{}'", "Error".red(), file_path_str);
                eprintln!("{}: Check the file path and try again.", "Hint".yellow());
                std::process::exit(1);
            }
        };
        let trimmed_content = content.trim();
        if trimmed_content.is_empty() {
            eprintln!("{}: File '{}' is empty.", "Error".red(), file_path_str);
            eprintln!("{}: Add some notes to the file first.", "Hint".yellow());
            std::process::exit(1);
        }
        let truncated: String = trimmed_content.chars().take(3000).collect();

        return Ok(format!(
            "You are a study assistant. Summarize the following notes into exactly 5 clear bullet points. Each bullet should be one concise sentence. Start each bullet with a dash (-).{}

Notes:
{}",
            lang_suffix, truncated
        ));
    }

    if let Some(file_path_str) = &cli.quiz_file {
        let path = PathBuf::from(file_path_str);
        if !path.exists() {
            eprintln!("{}: Could not read file '{}'", "Error".red(), file_path_str);
            eprintln!("{}: Check the file path and try again.", "Hint".yellow());
            std::process::exit(1);
        }
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("{}: Could not read file '{}'", "Error".red(), file_path_str);
                eprintln!("{}: Check the file path and try again.", "Hint".yellow());
                std::process::exit(1);
            }
        };
        let trimmed_content = content.trim();
        if trimmed_content.is_empty() {
            eprintln!("{}: File '{}' is empty.", "Error".red(), file_path_str);
            eprintln!("{}: Add some notes to the file first.", "Hint".yellow());
            std::process::exit(1);
        }
        let truncated: String = trimmed_content.chars().take(3000).collect();

        return Ok(format!(
            "You are a student quiz generator. Based on the following notes, generate exactly 5 quiz questions with answers. Format each as:
Q1: [question]
A1: [answer]
Q2: [question]
A2: [answer]
... and so on until Q5/A5.{}

Notes:
{}",
            lang_suffix, truncated
        ));
    }

    if let Some(topic) = &cli.quiz {
        let count = cli.count.unwrap_or(3);
        return Ok(format!(
            "Write {count} exam MCQs about '{topic}'. Use this exact format for each:
Q1: [question text]
a) [option]
b) [option]
c) [option]
d) [option]
Answer: [single letter a/b/c/d]
Q2: [question text]
...
Only output the questions. No intro, no explanation.{}", lang_suffix
        ));
    }

    if let Some(prompt) = &cli.prompt {
        return Ok(format!(
            "You are Radhe AI, a tiny offline terminal assistant for students. Be concise and practical.{}
User: {prompt}", lang_suffix
        ));
    }

    anyhow::bail!("no prompt provided. Try: radhe --code \"bubble sort in c\"")
}

fn run_inference(prompt: &str, model: &str, max_tokens: u32, mode: &str) -> Result<String> {
    let model_filename = if model.ends_with(".gguf") {
        model.to_string()
    } else {
        format!("{model}.gguf")
    };
    let model_path_buf = dirs::home_dir()
        .context("unable to find user home directory")?
        .join(".radhe")
        .join("models")
        .join(&model_filename);

    if !model_path_buf.exists() {
        eprintln!("{}: Model file '{}' not found in ~/.radhe/models/", "Error".red(), model_filename);
        eprintln!("{}: Run 'radhe models' to see available models.", "Hint".yellow());
        std::process::exit(1);
    }

    let model_path = model_path_buf.to_string_lossy().into_owned();

    debug_log(&format!("Prompt ({} chars): {}", prompt.len(), &prompt[..prompt.len().min(200)]));

    let prompt_with_delim = if mode == "fix" || mode == "chat" {
        prompt.to_string()
    } else {
        format!("{}\n\n### RESPONSE:\n", prompt)
    };

    let llama_bin = if cfg!(target_os = "windows") {
        "llama-completion.exe"
    } else {
        "llama-completion"
    };

    let max_tokens_str = max_tokens.to_string();
    let child = Command::new(llama_bin)
        .args([
            "-m",
            &model_path,
            "-p",
            &prompt_with_delim,
            "-n",
            &max_tokens_str,
            "-no-cnv",
            "--temp",
            "0.2",
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("NO_COLOR", "1")
        .spawn();

    let child = match child {
        Ok(c) => c,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            eprintln!("llama-completion not found. Run: radhe doctor");
            return Err(anyhow::anyhow!(e).context("llama-completion not found. Run: radhe doctor"));
        }
        Err(e) => return Err(e).context("failed to execute llama-completion"),
    };

    let output = child.wait_with_output().context("failed to wait on llama-completion child process")?;

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let stderr_str = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        anyhow::bail!(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let target_str = if stdout_str.trim().is_empty() { &stderr_str } else { &stdout_str };
    
    debug_log(&format!("Raw output ({} chars)", target_str.len()));
    
    // Filter out log/perf lines starting with "0."
    let cleaned_lines: Vec<&str> = target_str
        .lines()
        .filter(|line| !line.starts_with("0."))
        .collect();

    let cleaned_content = cleaned_lines.join("\n");

    let cleaned = if let Some(pos) = cleaned_content.find("### RESPONSE:") {
        let rest = &cleaned_content[pos + "### RESPONSE:".len()..];
        let response = if let Some(first_newline_idx) = rest.find('\n') {
            let before_newline = &rest[..first_newline_idx];
            if before_newline.trim().is_empty() {
                &rest[first_newline_idx + 1..]
            } else {
                rest
            }
        } else {
            rest
        };
        response.to_string()
    } else if let Some(pos) = cleaned_content.find("FIXED CODE:") {
        let rest = &cleaned_content[pos + "FIXED CODE:".len()..];
        let response = if let Some(first_newline_idx) = rest.find('\n') {
            let before_newline = &rest[..first_newline_idx];
            if before_newline.trim().is_empty() {
                &rest[first_newline_idx + 1..]
            } else {
                rest
            }
        } else {
            rest
        };
        response.to_string()
    } else {
        // Fall back to current stripping logic
        let prompt_normalized = prompt
            .replace("\r\n", "\n")
            .replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\t", "\t");
        let trimmed_prompt_normalized = prompt.trim()
            .replace("\r\n", "\n")
            .replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\t", "\t");

        let prompt_clean = prompt_normalized
            .replace("<|im_start|>", "")
            .replace("<|im_end|>", "");
        let trimmed_prompt_clean = trimmed_prompt_normalized
            .replace("<|im_start|>", "")
            .replace("<|im_end|>", "");

        let cleaned_content_clean = cleaned_content
            .replace("<|im_start|>", "")
            .replace("<|im_end|>", "");

        let end_pos = cleaned_content_clean.find(&prompt_clean)
            .map(|p| p + prompt_clean.len())
            .or_else(|| cleaned_content_clean.find(&trimmed_prompt_clean).map(|p| p + trimmed_prompt_clean.len()))
            .unwrap_or(0);

        let rest = &cleaned_content_clean[end_pos..];
        let response = if let Some(first_newline_idx) = rest.find('\n') {
            let before_newline = &rest[..first_newline_idx];
            if before_newline.trim().is_empty() {
                &rest[first_newline_idx + 1..]
            } else {
                rest
            }
        } else {
            rest
        };
        response.to_string()
    };

    let mut final_lines = Vec::new();
    for line in cleaned.lines() {
        let line_clean = line.replace("[end of text]", "");
        let line_trimmed = line_clean.trim();

        if line_trimmed.contains("### RESPONSE:") {
            continue;
        }

        if line_trimmed == "```c"
            || line_trimmed == "```cpp"
            || line_trimmed == "```python"
            || line_trimmed == "```rust"
            || line_trimmed == "```"
        {
            continue;
        }

        if mode == "code" || mode == "fix" {
            if line_trimmed.starts_with("Explanation:")
                || line_trimmed.starts_with("explanation:")
                || line_trimmed.starts_with("// Explanation")
                || line_trimmed.starts_with("// explanation")
                || line_trimmed.starts_with("# Explanation")
                || line_trimmed.starts_with("# explanation")
            {
                break;
            }
        }
        if line_trimmed.is_empty() {
            continue;
        }
        final_lines.push(line_clean);
    }
    let mut final_cleaned = final_lines.join("\n");

    if mode == "notes" {
        let mut seen_bullets = std::collections::HashSet::new();
        let mut deduplicated_lines = Vec::new();
        for line in final_lines {
            let line_trimmed = line.trim();
            if line_trimmed.is_empty() {
                continue;
            }
            let mut core = line.to_lowercase();
            core = core.replace("**", "");
            
            let mut trimmed_core = core.trim();
            loop {
                let prev_len = trimmed_core.len();
                if trimmed_core.starts_with('-') || trimmed_core.starts_with('*') {
                    trimmed_core = trimmed_core[1..].trim();
                }
                
                let mut has_digit_prefix = false;
                let mut digit_end = 0;
                for (i, c) in trimmed_core.char_indices() {
                    if c.is_ascii_digit() {
                        digit_end = i + c.len_utf8();
                    } else if c == '.' && digit_end > 0 {
                        trimmed_core = trimmed_core[i + c.len_utf8()..].trim();
                        has_digit_prefix = true;
                        break;
                    } else {
                        break;
                    }
                }
                
                if trimmed_core.len() == prev_len && !has_digit_prefix {
                    break;
                }
            }
            
            let core_text = trimmed_core.to_string();
            if seen_bullets.contains(&core_text) {
                continue;
            }
            seen_bullets.insert(core_text);
            deduplicated_lines.push(line);
        }
        final_cleaned = deduplicated_lines.join("\n");
    }

    Ok(final_cleaned.trim().to_string())
}

fn init_dirs() -> Result<()> {
    fs::create_dir_all("models")?;
    fs::create_dir_all("installer")?;
    fs::create_dir_all(".radhe")?;
    Ok(())
}

fn run_doctor(active_model: &str, active_lang: &str) {
    use colored::Colorize;

    let version = env!("CARGO_PKG_VERSION");
    println!("Radhe AI v{}", version);
    println!("Running diagnostics...");

    let mut all_ok = true;

    let llama_bin = if cfg!(target_os = "windows") {
        "llama-completion.exe"
    } else {
        "llama-completion"
    };

    // 1. Check llama-completion in PATH
    match Command::new(llama_bin).arg("--help").output() {
        Ok(_) => {
            println!("{}", format!("✓ {} found", llama_bin).green());
        }
        Err(_) => {
            println!("{}", format!("✗ {} not found", llama_bin).red());
            all_ok = false;
        }
    }

    // 2. Check active model file in ~/.radhe/models/
    let model_filename = if active_model.ends_with(".gguf") {
        active_model.to_string()
    } else {
        format!("{active_model}.gguf")
    };

    let model_path = dirs::home_dir()
        .map(|p| p.join(".radhe").join("models").join(&model_filename));

    if let Some(path) = &model_path {
        if path.exists() {
            println!("{}", format!("✓ Active model: {} (found)", active_model).green());
        } else {
            println!("{}", format!("✗ Active model: {} (NOT FOUND — run: radhe models)", active_model).red());
            all_ok = false;
        }
    } else {
        println!("{}", "✗ Expected model path: unable to resolve home directory".red());
        all_ok = false;
    }

    // 3. Model warning if not default
    if active_model != "Qwen2.5-Coder-1.5B-Instruct-Q4_K_M.gguf" && active_model != "Qwen2.5-Coder-1.5B-Instruct-Q4_K_M"
        && active_model != "qwen2.5-coder-0.5b-instruct-q4_k_m.gguf" && active_model != "qwen2.5-coder-0.5b-instruct-q4_k_m" {
        println!("{}", format!("! Warning: Using non-default model ({})", active_model).yellow());
    }

    // 5. Print language setting
    let lang_label = match active_lang {
        "hi" => "Hindi",
        "hinglish" => "Hinglish",
        _ => "English",
    };
    println!("{}", format!("✓ Language: {} ({})", lang_label, active_lang).green());
    if active_lang != "en" {
        println!("{}", format!("  Tip: Reset with --set-lang en").dimmed());
    }

    // 4. Print final status
    if all_ok {
        println!("{}", "All systems operational.".green());
    } else {
        println!("{}", "✗ Systems check failed. Check the errors above.".red());
    }
}

fn run_repl(model: &str, lang: &str) -> Result<()> {
    use colored::Colorize;
    use std::io::{self, BufRead, Write};

    // Set up Ctrl+C handler with a friendly cyan/yellow exit message
    ctrlc::set_handler(move || {
        println!("\n\n{}", "Goodbye! Hope Radhe AI helped you today!".cyan().bold());
        std::process::exit(0);
    })
    .context("Error setting Ctrl-C handler")?;

    // Print welcome header
    println!("{}", format!("Radhe AI v{} — Offline Terminal Assistant", env!("CARGO_PKG_VERSION")).cyan().bold());
    println!("{}", "Type your question, or prefix with --code / --explain / --notes".yellow());
    println!("{}", "/exit to quit, /clear to clear screen".yellow());
    println!();

    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        print!("{}", ">>> ".green().bold());
        io::stdout().flush().context("failed to flush stdout")?;

        let mut input = String::new();
        let bytes_read = reader.read_line(&mut input).context("failed to read from stdin")?;
        
        if bytes_read == 0 {
            println!("\n{}", "Goodbye! Hope Radhe AI helped you today!".cyan().bold());
            break;
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed == "/exit" {
            println!("{}", "Goodbye! Hope Radhe AI helped you today!".cyan().bold());
            break;
        }

        if trimmed == "/clear" {
            for _ in 0..50 {
                println!();
            }
            continue;
        }

        // Parse input prefixes
        let (prompt_text, mode, max_tokens) = if trimmed.starts_with("--code ") {
            let task = trimmed["--code ".len()..].trim();
            let has_lang_hint = task
                .to_lowercase()
                .split(|c: char| !c.is_alphanumeric() && c != '+' && c != '#')
                .any(|word| {
                    matches!(
                        word,
                        "c" | "c++"
                            | "cpp"
                            | "c#"
                            | "csharp"
                            | "rust"
                            | "python"
                            | "java"
                            | "javascript"
                            | "js"
                            | "typescript"
                            | "ts"
                            | "go"
                            | "golang"
                            | "ruby"
                            | "php"
                            | "swift"
                            | "kotlin"
                            | "bash"
                            | "shell"
                            | "powershell"
                            | "sql"
                            | "html"
                            | "css"
                            | "assembly"
                    )
                });

            let mut task_str = task.to_string();
            if has_lang_hint {
                task_str.push_str(", respect the exact language specified.");
            }

            let prompt = format!(
                "You are a coding assistant. Return ONLY valid compilable code with zero explanation. No markdown, no backticks, no comments. Just raw code.
Task: {task_str}"
            );
            (prompt, "code", 300)
        } else {
            let lang_suffix = if lang != "en" {
                format!("\n\n{}", lang_system_prompt(lang))
            } else {
                String::new()
            };

            if trimmed.starts_with("--explain ") {
                let topic = trimmed["--explain ".len()..].trim();
                let prompt = format!(
                    "Explain '{topic}' in exactly 5 bullet points for a beginner programmer. Each bullet must be one sentence. Stop after 5 bullets. Do not repeat yourself.\n\nExplanation:{lang_suffix}"
                );
                (prompt, "explain", 200)
            } else if trimmed.starts_with("--notes ") {
                let topic = trimmed["--notes ".len()..].trim();
                let prompt = format!(
                    "Give exactly 6 bullet points about '{topic}' for a student. Format strictly as:
- [fact 1]
- [fact 2]
- [fact 3]
- [fact 4]
- [fact 5]
- [fact 6]
Each bullet = one unique fact. Max 15 words per bullet. Start directly with the first bullet, no intro paragraph.{lang_suffix}"
                );
                (prompt, "notes", 150)
            } else {
                let prompt = format!(
                    "Explain '{trimmed}' in exactly 5 bullet points for a beginner programmer. Each bullet must be one sentence. Stop after 5 bullets. Do not repeat yourself.\n\nExplanation:{lang_suffix}"
                );
                (prompt, "explain", 200)
            }
        };

        // Call run_inference
        match run_inference(&prompt_text, model, max_tokens, mode) {
            Ok(output) => {
                println!("{output}");
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
        println!();
    }

    Ok(())
}

fn starts_with_q_marker(line: &str) -> bool {
    let s = line.trim_start();
    if !s.starts_with('Q') {
        return false;
    }
    let rest = &s[1..];
    let mut chars = rest.chars();
    let mut has_digits = false;
    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            has_digits = true;
        } else if c == ':' && has_digits {
            return true;
        } else {
            return false;
        }
    }
    false
}

fn run_quiz(output: &str) {
    let mut question_blocks: Vec<Vec<String>> = Vec::new();
    let mut current_block: Vec<String> = Vec::new();

    for line in output.lines() {
        if starts_with_q_marker(line) {
            if !current_block.is_empty() {
                question_blocks.push(current_block);
                current_block = Vec::new();
            }
        }
        if starts_with_q_marker(line) || !current_block.is_empty() {
            current_block.push(line.to_string());
        }
    }
    if !current_block.is_empty() {
        question_blocks.push(current_block);
    }

    let mut correct = 0;
    let mut valid_questions = 0;

    use std::io::{self, Write};

    for q_block in question_blocks {
        let mut expected_char: Option<char> = None;
        let mut answer_line_idx: Option<usize> = None;

        for (idx, line) in q_block.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.to_lowercase().starts_with("answer:") {
                let parts: Vec<&str> = trimmed.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let ans_str = parts[1].trim().to_lowercase();
                    if let Some(c) = ans_str.chars().next() {
                        expected_char = Some(c);
                    }
                }
                answer_line_idx = Some(idx);
                break;
            }
        }

        // Print question + options only (REMOVE the Answer: line)
        for (idx, line) in q_block.iter().enumerate() {
            if Some(idx) != answer_line_idx {
                println!("{line}");
            }
        }

        // Read user input, trim + lowercase + take first char
        print!("Your answer: ");
        let _ = io::stdout().flush();
        let mut user_input = String::new();
        if io::stdin().read_line(&mut user_input).is_err() {
            println!("Error reading input.");
            continue;
        }
        let user_char = user_input.trim().to_lowercase().chars().next();

        let is_correct = match (expected_char, user_char) {
            (Some(e), Some(u)) => e == u,
            _ => false,
        };

        if let Some(ans_char) = expected_char {
            if is_correct {
                println!("✓ Correct!");
            } else {
                println!("✗ Wrong. Answer was: {ans_char}");
            }
        } else {
            println!("✗ Wrong. (No valid answer key found)");
        }

        // Increment score only if answer letter is a/b/c/d (valid)
        let is_valid = match expected_char {
            Some('a') | Some('b') | Some('c') | Some('d') => true,
            _ => false,
        };

        if is_valid {
            valid_questions += 1;
            if is_correct {
                correct += 1;
            }
        }
        println!();
    }

    println!("Score: {correct}/{valid_questions}");
}

fn run_models(active_model: &str) -> Result<()> {
    let models_dir = dirs::home_dir()
        .context("unable to find user home directory")?
        .join(".radhe")
        .join("models");

    println!("Models in {}:", models_dir.display());
    println!();

    if !models_dir.exists() {
        println!("No models directory found.");
        return Ok(());
    }

    let active_model_filename = if active_model.ends_with(".gguf") {
        active_model.to_string()
    } else {
        format!("{active_model}.gguf")
    };

    let mut found = false;
    for entry in fs::read_dir(&models_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "gguf" {
                    if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                        found = true;
                        let metadata = entry.metadata()?;
                        let size_bytes = metadata.len();
                        let size_mb = size_bytes as f64 / 1024.0 / 1024.0;
                        let is_active = filename.to_lowercase() == active_model_filename.to_lowercase();
                        if is_active {
                            println!("* {} ({:.0} MB) [active]", filename, size_mb);
                        } else {
                            println!("  {} ({:.0} MB)", filename, size_mb);
                        }
                    }
                }
            }
        }
    }

    if !found {
        println!("No .gguf models found.");
    }

    Ok(())
}

fn run_update() -> Result<()> {
    let current_version = env!("CARGO_PKG_VERSION");
    println!("Checking for updates...");
    // Fetch latest version tag from GitHub API
    let api_url = "https://api.github.com/repos/DevWizard-Vandan/radhe-ai/releases/latest";
    let latest_tag_output = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args(["-Command", &format!(
                "Invoke-RestMethod -Uri '{}' | Select-Object -ExpandProperty tag_name", api_url
            )])
            .output()
    } else {
        Command::new("curl")
            .args(["-fsSL", "-H", "Accept: application/vnd.github+json",
                   "-H", "X-GitHub-Api-Version: 2022-11-28", api_url])
            .output()
    };
    let output = match latest_tag_output {
        Ok(out) => out,
        Err(_) => {
            eprintln!("{}: Could not reach GitHub API.", "Error".red());
            eprintln!("{}: Check your internet connection and try again.", "Hint".yellow());
            std::process::exit(1);
        }
    };
    if !output.status.success() {
        eprintln!("{}: Could not reach GitHub API.", "Error".red());
        eprintln!("{}: Check your internet connection and try again.", "Hint".yellow());
        std::process::exit(1);
    }
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    // On Linux/macOS, parse tag_name from JSON using basic string search
    let mut latest_version = if cfg!(target_os = "windows") {
        stdout_str.trim().to_string()
    } else {
        // Parse "tag_name":"v0.7.0" from JSON
        let json = stdout_str.trim();
        let key = "\"tag_name\":\"";
        if let Some(start) = json.find(key) {
            let rest = &json[start + key.len()..];
            if let Some(end) = rest.find('"') {
                rest[..end].to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    };
    if latest_version.starts_with('v') {
        latest_version = latest_version[1..].to_string();
    }
    if latest_version.is_empty() {
        eprintln!("{}: Could not parse latest version from GitHub API.", "Error".red());
        eprintln!("{}: Try again later or check github.com/DevWizard-Vandan/radhe-ai/releases", "Hint".yellow());
        std::process::exit(1);
    }
    debug_log(&format!("Latest version from API: {}", latest_version));
    if current_version == latest_version {
        println!("Radhe AI is already up to date (v{})", current_version);
        return Ok(());
    }
    println!("Update available: v{} → v{}", current_version, latest_version);
    println!("Downloading new binary...");
    let current_exe = std::env::current_exe()?;
    let exe_dir = current_exe.parent().context("failed to get current exe directory")?;
    let (new_exe_name, download_filename) = if cfg!(target_os = "windows") {
        ("radhe_new.exe", "radhe.exe")
    } else {
        ("radhe_new", "radhe")
    };
    let new_exe_path = exe_dir.join(new_exe_name);
    let new_exe_path_str = new_exe_path.to_string_lossy().to_string();
    let download_url = format!(
        "https://github.com/DevWizard-Vandan/radhe-ai/releases/latest/download/{}",
        download_filename
    );
    let download_success = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args(["-Command", &format!(
                "Invoke-WebRequest -Uri '{}' -OutFile '{}'", download_url, new_exe_path_str
            )])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    } else {
        Command::new("curl")
            .args(["-fsSL", "-o", &new_exe_path_str, &download_url])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    };
    if !download_success {
        eprintln!("{}: Failed to download new binary.", "Error".red());
        eprintln!("{}: Try again later or download manually from github.com/DevWizard-Vandan/radhe-ai/releases", "Hint".yellow());
        std::process::exit(1);
    }
    // Make executable on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&new_exe_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&new_exe_path, perms)?;
    }
    // Replace binary
    let old_suffix = if cfg!(target_os = "windows") { "radhe_old.exe" } else { "radhe_old" };
    let old_exe_path = exe_dir.join(old_suffix);
    if old_exe_path.exists() { let _ = fs::remove_file(&old_exe_path); }
    fs::rename(&current_exe, &old_exe_path).context("failed to rename current binary")?;
    fs::rename(&new_exe_path, &current_exe).context("failed to rename new binary")?;
    println!("Radhe AI updated to v{}! Restart your terminal.", latest_version);
    Ok(())
}

fn run_chat(active_model: &str, active_lang: &str) -> Result<()> {
    println!("Radhe AI - Chat Mode");
    println!("Type 'exit' to quit.");
    println!("──────────────────────");

    let mut history: Vec<(String, String)> = vec![]; // (user, assistant) pairs
    use std::io::{self, Write};

    loop {
        print!("You: ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input.");
            continue;
        }
        let input = input.trim().to_string();
        if input == "exit" || input == "quit" {
            println!("Goodbye!");
            break;
        }
        if input.is_empty() {
            continue;
        }

        // Build rolling prompt from history with language support
        let lang_instruction = if active_lang != "en" {
            format!(" {}", lang_system_prompt(active_lang))
        } else {
            String::new()
        };
        let mut prompt = format!("<|im_start|>system\nYou are Radhe, a concise AI assistant for students. Give short, direct answers. No bullet points unless asked. No headers. Maximum 3 sentences per response.{}<|im_end|>\n", lang_instruction);
        for (u, a) in &history {
            prompt.push_str(&format!("<|im_start|>user\n{}<|im_end|>\n<|im_start|>assistant\n{}<|im_end|>\n", u, a));
        }
        prompt.push_str(&format!("<|im_start|>user\n{}<|im_end|>\n<|im_start|>assistant\n", input));

        // Run inference (same as existing modes)
        print!("Radhe: ");
        let _ = io::stdout().flush();
        
        let response = run_inference(&prompt, active_model, 300, "chat")
            .context("failed to run local inference")?;

        let cutoffs = ["### END", "###", "\nUser:", "\nYou:"];
        let mut response = response.trim().to_string();
        for cutoff in &cutoffs {
            if let Some(idx) = response.find(cutoff) {
                response = response[..idx].trim().to_string();
            }
        }

        // Store in history, cap at last 6 turns to avoid context overflow
        history.push((input, response.clone()));
        if history.len() > 6 {
            history.remove(0);
        }
        println!("{}", response);
        println!();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_prompt_code() {
        let cli = Cli {
            command: None,
            version: false,
            prompt: None,
            code: Some("bubble sort in c".to_string()),
            explain: None,
            notes: None,
            fix: None,
            summarize: None,
            chat: false,
            quiz: None,
            quiz_file: None,
            count: None,
            model: None,
            max_tokens: None,
            pack: None,
            list_packs: false,
            create_pack: false,
            delete_pack: None,
            lang: None,
            set_lang: None,
        };
        let p = build_prompt(&cli, "en").unwrap();
        assert!(p.contains("coding assistant"), "should contain coding assistant");
        assert!(p.contains("bubble sort in c"), "should contain prompt text");
    }

    #[test]
    fn test_build_prompt_explain() {
        let cli = Cli {
            command: None,
            version: false,
            prompt: None,
            code: None,
            explain: Some("recursion".to_string()),
            notes: None,
            fix: None,
            summarize: None,
            chat: false,
            quiz: None,
            quiz_file: None,
            count: None,
            model: None,
            max_tokens: None,
            pack: None,
            list_packs: false,
            create_pack: false,
            delete_pack: None,
            lang: None,
            set_lang: None,
        };
        let p = build_prompt(&cli, "en").unwrap();
        assert!(p.contains("Explain 'recursion' in exactly 5 bullet points"), "should format explanation prompt");
    }
}




