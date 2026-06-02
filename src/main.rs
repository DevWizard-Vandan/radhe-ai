use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
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

#[derive(serde::Serialize, serde::Deserialize, Default, Debug)]
struct RadheConfig {
    model: Option<String>,
    max_tokens: Option<u32>,
    lang: Option<String>,
    mode: Option<String>,
    difficulty: Option<String>,
    profile: Option<String>,
    stats_enabled: Option<bool>,
    packs_enabled: Option<bool>,
    shell_enabled: Option<bool>,
    modes_enabled: Option<bool>,
    difficulty_enabled: Option<bool>,
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

    /// Study mode: 'normal' (default), 'exam' (shorter, direct, no fluff), 'revision' (bullet-style concise memory aids)
    #[arg(long = "mode", value_name = "MODE")]
    mode: Option<String>,

    /// Quiz difficulty: 'easy', 'medium' (default), 'hard'
    #[arg(long = "difficulty", value_name = "DIFFICULTY")]
    difficulty: Option<String>,

    /// Set default study mode in config: 'normal', 'exam', or 'revision'
    #[arg(long = "set-mode", value_name = "MODE")]
    set_mode: Option<String>,

    /// Set default quiz difficulty in config: 'easy', 'medium', or 'hard'
    #[arg(long = "set-difficulty", value_name = "DIFFICULTY")]
    set_difficulty: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init,
    Doctor,
    Models,
    Update,
    Stats {
        #[arg(long)]
        reset: bool,
    },
    Shell,
    Setup,
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

    // Handle --set-mode early
    if let Some(ref new_mode) = cli.set_mode {
        let valid = ["normal", "exam", "revision"];
        if !valid.contains(&new_mode.as_str()) {
            eprintln!("{}: Invalid study mode '{}'. Valid options: normal, exam, revision", "Error".red(), new_mode);
            eprintln!("{}: Use --set-mode normal, --set-mode exam, or --set-mode revision", "Hint".yellow());
            std::process::exit(1);
        }
        // Read existing config and update mode line
        let content = fs::read_to_string(&config_path).unwrap_or_default();
        let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
        let mut found_mode = false;
        for line in lines.iter_mut() {
            if line.starts_with("mode") && line.contains('=') {
                *line = format!("mode = \"{}\"", new_mode);
                found_mode = true;
                break;
            }
        }
        if !found_mode {
            lines.push(format!("mode = \"{}\"", new_mode));
        }
        fs::write(&config_path, lines.join("\n") + "\n")?;
        println!("Default study mode set to '{}'. All future responses will use this mode.", new_mode);
        std::process::exit(0);
    }

    // Handle --set-difficulty early
    if let Some(ref new_diff) = cli.set_difficulty {
        let valid = ["easy", "medium", "hard"];
        if !valid.contains(&new_diff.as_str()) {
            eprintln!("{}: Invalid quiz difficulty '{}'. Valid options: easy, medium, hard", "Error".red(), new_diff);
            eprintln!("{}: Use --set-difficulty easy, --set-difficulty medium, or --set-difficulty hard", "Hint".yellow());
            std::process::exit(1);
        }
        // Read existing config and update difficulty line
        let content = fs::read_to_string(&config_path).unwrap_or_default();
        let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
        let mut found_diff = false;
        for line in lines.iter_mut() {
            if line.starts_with("difficulty") && line.contains('=') {
                *line = format!("difficulty = \"{}\"", new_diff);
                found_diff = true;
                break;
            }
        }
        if !found_diff {
            lines.push(format!("difficulty = \"{}\"", new_diff));
        }
        fs::write(&config_path, lines.join("\n") + "\n")?;
        println!("Default quiz difficulty set to '{}'. All future quizzes will use this difficulty.", new_diff);
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

    // Resolve study mode: CLI flag > config.toml > default "normal"
    let active_mode = cli.mode
        .clone()
        .or_else(|| config.mode.clone())
        .unwrap_or_else(|| "normal".to_string());

    if !["normal", "exam", "revision"].contains(&active_mode.as_str()) {
        eprintln!("{}: Invalid study mode '{}'. Valid options: normal, exam, revision", "Error".red(), active_mode);
        std::process::exit(1);
    }

    // Resolve quiz difficulty: CLI flag > config.toml > default "medium"
    let active_difficulty = cli.difficulty
        .clone()
        .or_else(|| config.difficulty.clone())
        .unwrap_or_else(|| "medium".to_string());

    if !["easy", "medium", "hard"].contains(&active_difficulty.as_str()) {
        eprintln!("{}: Invalid quiz difficulty '{}'. Valid options: easy, medium, hard", "Error".red(), active_difficulty);
        std::process::exit(1);
    }

    let active_profile = config.profile.clone().unwrap_or_else(|| "standard".to_string());

    debug_log(&format!("Config loaded — model: {}, max_tokens: {}, lang: {}, mode: {}, difficulty: {}", active_model, active_max_tokens, active_lang, active_mode, active_difficulty));

    if let Some(ref pack_name) = cli.pack {
        if config.packs_enabled == Some(false) {
            println!("Packs are disabled. Run 'radhe setup' to enable them.");
            std::process::exit(0);
        }
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

        let stats_path = config_dir.join("stats.toml");
        let mut stats = load_stats(&stats_path);
        stats.total_commands += 1;
        *stats.pack_usage.entry(pack_name.to_lowercase()).or_insert(0) += 1;
        let _ = save_stats(&stats_path, &stats);

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
            run_doctor(&active_model, &active_lang, &active_mode, &active_difficulty, &active_profile);
            return Ok(());
        }
        Some(Commands::Shell) => {
            if config.shell_enabled == Some(false) {
                println!("Shell is disabled. Run 'radhe setup' to enable it.");
                std::process::exit(0);
            }
            run_power_shell(&active_model, &active_lang, &active_difficulty, &active_mode)?;
            return Ok(());
        }
        Some(Commands::Setup) => {
            run_setup(&config_path, &active_mode, &active_difficulty, &active_lang)?;
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
        Some(Commands::Stats { reset }) => {
            let stats_path = config_dir.join("stats.toml");
            if reset {
                use std::io::{self, Write};
                print!("Wipe all local usage statistics? [y/N]: ");
                io::stdout().flush().ok();
                let mut confirm = String::new();
                io::stdin().read_line(&mut confirm).ok();
                if confirm.trim().to_lowercase() == "y" {
                    if stats_path.exists() {
                        fs::remove_file(&stats_path)?;
                    }
                    println!("Statistics wiped.");
                } else {
                    println!("Aborted.");
                }
            } else {
                let stats = load_stats(&stats_path);
                println!("Radhe AI — Usage Stats");
                println!("─────────────────────────────");
                println!("Total commands run : {}", stats.total_commands);
                println!("Explains           : {}", stats.explain_count);
                println!("Code generations   : {}", stats.code_count);
                println!("Quizzes            : {}", stats.quiz_count);
                println!("Notes              : {}", stats.notes_count);
                println!("Summaries          : {}", stats.summarize_count);
                println!("Chats              : {}", stats.chat_count);
                if !stats.pack_usage.is_empty() {
                    println!("Pack usage:");
                    let mut sorted_packs: Vec<(&String, &u64)> = stats.pack_usage.iter().collect();
                    sorted_packs.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
                    for (pack, count) in sorted_packs {
                        println!("  {:<8}: {}", pack, count);
                    }
                }
            }
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
        run_repl(&active_model, &active_lang, &active_difficulty, &active_mode, &active_profile)?;
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
        let stats_path = config_dir.join("stats.toml");
        let mut stats = load_stats(&stats_path);
        stats.total_commands += 1;
        stats.chat_count += 1;
        let _ = save_stats(&stats_path, &stats);

        run_chat(&active_model, &active_lang, &active_mode)?;
        return Ok(());
    }

    let prompt = build_prompt(&cli, &active_lang, &active_difficulty, &active_mode)?;
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

    let stats_path = config_dir.join("stats.toml");
    let mut stats = load_stats(&stats_path);
    stats.total_commands += 1;
    match mode {
        "code" => stats.code_count += 1,
        "explain" => stats.explain_count += 1,
        "notes" => stats.notes_count += 1,
        "quiz" | "quiz_file" => stats.quiz_count += 1,
        "summarize" => stats.summarize_count += 1,
        _ => {}
    }
    let _ = save_stats(&stats_path, &stats);

    if mode == "quiz" {
        run_quiz(&output);
    } else {
        println!("{output}");
    }
    Ok(())
}

fn build_prompt(cli: &Cli, lang: &str, difficulty: &str, study_mode: &str) -> Result<String> {
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
        let prompt_body = match study_mode {
            "exam" => format!("Explain '{topic}' for a beginner programmer. Provide very short, direct, exam-focused answers with zero fluff. Stop after 3 sentences."),
            "revision" => format!("Explain '{topic}' for a beginner programmer using bullet-style concise memory aids and quick revision facts. Keep them extremely short and punchy."),
            _ => format!("Explain '{topic}' in exactly 5 bullet points for a beginner programmer. Each bullet must be one sentence. Stop after 5 bullets. Do not repeat yourself."),
        };
        return Ok(format!("{}{}\n\nExplanation:", prompt_body, lang_suffix));
    }

    if let Some(topic) = &cli.notes {
        let prompt_body = match study_mode {
            "exam" => format!(
                "Give exactly 6 bullet points about '{topic}' for a student. Focus strictly on exam-relevant facts, direct and with zero fluff. Format strictly as:
- [fact 1]
- [fact 2]
- [fact 3]
- [fact 4]
- [fact 5]
- [fact 6]
Each bullet = one unique fact. Max 15 words per bullet. Start directly with the first bullet, no intro paragraph."
            ),
            "revision" => format!(
                "Give exactly 6 bullet points about '{topic}' for a student. Use bullet-style concise memory aids and quick revision facts that are extremely punchy. Format strictly as:
- [fact 1]
- [fact 2]
- [fact 3]
- [fact 4]
- [fact 5]
- [fact 6]
Each bullet = one unique fact. Max 15 words per bullet. Start directly with the first bullet, no intro paragraph."
            ),
            _ => format!(
                "Give exactly 6 bullet points about '{topic}' for a student. Format strictly as:
- [fact 1]
- [fact 2]
- [fact 3]
- [fact 4]
- [fact 5]
- [fact 6]
Each bullet = one unique fact. Max 15 words per bullet. Start directly with the first bullet, no intro paragraph."
            ),
        };
        return Ok(format!("{}{}", prompt_body, lang_suffix));
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

        let prompt_body = match study_mode {
            "exam" => "You are an exam prep assistant. Summarize the following notes into extremely direct, short exam answers with zero fluff. No intro/outro.",
            "revision" => "You are a revision assistant. Summarize the following notes into bullet-style concise memory aids and quick revision facts. Extremely punchy.",
            _ => "You are a study assistant. Summarize the following notes into exactly 5 clear bullet points. Each bullet should be one concise sentence. Start each bullet with a dash (-).",
        };

        return Ok(format!(
            "{}{}

Notes:
{}",
            prompt_body, lang_suffix, truncated
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

        let difficulty_instruction = match difficulty {
            "easy" => "The questions must be simple and easy, focusing on basic recall and literal facts from the notes.",
            "hard" => "The questions must be advanced and hard, testing critical thinking, synthesis, and deep implications or complex details of the notes.",
            _ => "The questions must be of moderate/medium difficulty, testing conceptual understanding and main ideas from the notes.",
        };

        return Ok(format!(
            "You are a student quiz generator. Based on the following notes, generate exactly 5 quiz questions with answers. {difficulty_instruction} Format each as:
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
        let difficulty_instruction = match difficulty {
            "easy" => "The questions must be simple and easy, testing basic facts and introductory concepts with straightforward options.",
            "hard" => "The questions must be advanced and hard, testing deep analytical skills, complex scenarios, and edge cases with subtle and challenging distractors.",
            _ => "The questions must be of moderate/medium difficulty, testing standard conceptual understanding and application.",
        };
        return Ok(format!(
            "Write {count} exam MCQs about '{topic}'. {difficulty_instruction} Use this exact format for each:
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
        let assistant_description = match study_mode {
            "exam" => "You are Radhe AI, a tiny offline terminal assistant for students. Provide very short, direct answers with zero fluff.",
            "revision" => "You are Radhe AI, a tiny offline terminal assistant for students. Respond using bullet-style concise memory aids.",
            _ => "You are Radhe AI, a tiny offline terminal assistant for students. Be concise and practical.",
        };
        return Ok(format!(
            "{}{}
User: {prompt}", assistant_description, lang_suffix
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

fn run_doctor(active_model: &str, active_lang: &str, active_mode: &str, active_difficulty: &str, profile: &str) {
    use colored::Colorize;

    let version = env!("CARGO_PKG_VERSION");
    println!("Radhe AI v{}", version);
    println!("Running diagnostics...");
    println!("{}", format!("✓ Profile: {}", profile).green());

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

    // 6. Print study mode setting
    println!("{}", format!("✓ Study Mode: {}", active_mode).green());

    // 7. Print quiz difficulty setting
    println!("{}", format!("✓ Quiz Difficulty: {}", active_difficulty).green());

    // 8. Print shell availability
    println!("{}", "✓ Shell: available".green());

    // 4. Print final status
    if all_ok {
        println!("{}", "All systems operational.".green());
    } else {
        println!("{}", "✗ Systems check failed. Check the errors above.".red());
    }
}

fn run_repl(model: &str, lang: &str, difficulty: &str, study_mode: &str, profile: &str) -> Result<()> {
    use colored::Colorize;
    use std::io::{self, BufRead, Write};

    // Set up Ctrl+C handler with a friendly cyan/yellow exit message
    ctrlc::set_handler(move || {
        println!("\n\n{}", "Goodbye! Hope Radhe AI helped you today!".cyan().bold());
        std::process::exit(0);
    })
    .context("Error setting Ctrl-C handler")?;

    // Print welcome header
    let version = env!("CARGO_PKG_VERSION");
    let lang_label = match lang {
        "hi" => "Hindi",
        "hinglish" => "Hinglish",
        _ => "English",
    };
    println!("{}", "╔══════════════════════════════════════════╗".cyan());
    println!("{}", format!("║        Radhe AI v{} — Offline AI      ║", version).cyan());
    println!("{}", "╚══════════════════════════════════════════╝".cyan());
    println!(" {} : {}", "Profile".green(), profile);
    println!(" {}   : {}", "Model".green(), model);
    println!(" {}    : {}   {}", "Mode".green(), study_mode, "(change: --set-mode exam)".yellow());
    println!(" {}: {}  {}", "Difficulty".green(), difficulty, "(change: --set-difficulty hard)".yellow());
    println!(" {}: {}   {}", "Language".green(), lang_label, "(change: --set-lang hi)".yellow());
    println!(" Commands you can use right now:");
    println!("  {}   Explain a concept", "--explain <topic>".yellow());
    println!("  {}     Study notes in bullets", "--notes <topic>".yellow());
    println!("  {}       Generate code", "--code <task>".yellow());
    println!("  {}               Quit  |  {}  Clear screen", "/exit".yellow(), "/clear".yellow());
    println!("{}", "──────────────────────────────────────────".cyan());
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
        let mut prefix_matched = true;
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
                let prompt = match study_mode {
                    "exam" => format!(
                        "Explain '{topic}' for a beginner programmer. Provide very short, direct, exam-focused answers with zero fluff. Stop after 3 sentences.{lang_suffix}"
                    ),
                    "revision" => format!(
                        "Explain '{topic}' for a beginner programmer using bullet-style concise memory aids and quick revision facts. Keep them extremely short and punchy.{lang_suffix}"
                    ),
                    _ => format!(
                        "Explain '{topic}' in exactly 5 bullet points for a beginner programmer. Each bullet must be one sentence. Stop after 5 bullets. Do not repeat yourself.\n\nExplanation:{lang_suffix}"
                    ),
                };
                (prompt, "explain", 200)
            } else if trimmed.starts_with("--notes ") {
                let topic = trimmed["--notes ".len()..].trim();
                let prompt = match study_mode {
                    "exam" => format!(
                        "Give exactly 6 bullet points about '{topic}' for a student. Focus strictly on exam-relevant facts, direct and with zero fluff. Format strictly as:
- [fact 1]
- [fact 2]
- [fact 3]
- [fact 4]
- [fact 5]
- [fact 6]
Each bullet = one unique fact. Max 15 words per bullet. Start directly with the first bullet, no intro paragraph.{lang_suffix}"
                    ),
                    "revision" => format!(
                        "Give exactly 6 bullet points about '{topic}' for a student. Use bullet-style concise memory aids and quick revision facts that are extremely punchy. Format strictly as:
- [fact 1]
- [fact 2]
- [fact 3]
- [fact 4]
- [fact 5]
- [fact 6]
Each bullet = one unique fact. Max 15 words per bullet. Start directly with the first bullet, no intro paragraph.{lang_suffix}"
                    ),
                    _ => format!(
                        "Give exactly 6 bullet points about '{topic}' for a student. Format strictly as:
- [fact 1]
- [fact 2]
- [fact 3]
- [fact 4]
- [fact 5]
- [fact 6]
Each bullet = one unique fact. Max 15 words per bullet. Start directly with the first bullet, no intro paragraph.{lang_suffix}"
                    ),
                };
                (prompt, "notes", 150)
            } else {
                prefix_matched = false;
                let prompt = match study_mode {
                    "exam" => format!(
                        "Explain '{trimmed}' for a beginner programmer. Provide very short, direct, exam-focused answers with zero fluff. Stop after 3 sentences.{lang_suffix}"
                    ),
                    "revision" => format!(
                        "Explain '{trimmed}' for a beginner programmer using bullet-style concise memory aids and quick revision facts. Keep them extremely short and punchy.{lang_suffix}"
                    ),
                    _ => format!(
                        "Explain '{trimmed}' in exactly 5 bullet points for a beginner programmer. Each bullet must be one sentence. Stop after 5 bullets. Do not repeat yourself.\n\nExplanation:{lang_suffix}"
                    ),
                };
                (prompt, "explain", 200)
            }
        };

        // Call run_inference
        match run_inference(&prompt_text, model, max_tokens, mode) {
            Ok(output) => {
                if let Some(home) = dirs::home_dir() {
                    let stats_path = home.join(".radhe").join("stats.toml");
                    let mut stats = load_stats(&stats_path);
                    stats.total_commands += 1;
                    match mode {
                        "code" => stats.code_count += 1,
                        "explain" => stats.explain_count += 1,
                        "notes" => stats.notes_count += 1,
                        _ => {}
                    }
                    let _ = save_stats(&stats_path, &stats);
                }
                println!("{output}");

                if !prefix_matched {
                    let word_count = trimmed.split_whitespace().count();
                    if word_count < 4 {
                        println!();
                        println!("{}", "Tip: Use --explain <topic> for structured explanations, --notes <topic> for bullet points.".yellow());
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
        println!();
    }

    Ok(())
}

fn run_power_shell(model: &str, lang: &str, initial_difficulty: &str, initial_mode: &str) -> Result<()> {
    use colored::Colorize;
    use std::io::{self, BufRead, Write};

    // Set up Ctrl+C handler with a friendly cyan/yellow exit message
    ctrlc::set_handler(move || {
        println!("\n\n{}", "Goodbye! Hope Radhe AI helped you today!".cyan().bold());
        std::process::exit(0);
    })
    .context("Error setting Ctrl-C handler")?;
    let mut current_difficulty = initial_difficulty.to_string();
    let mut current_mode = initial_mode.to_string();
    let mut history: Vec<String> = Vec::new();

    // Print welcome header
    let version = env!("CARGO_PKG_VERSION");
    println!("{}", "╔══════════════════════════════════════════╗".cyan());
    println!("{}", format!("║      Radhe AI v{} — Power Shell       ║", version).cyan());
    println!("{}", "╚══════════════════════════════════════════╝".cyan());
    println!(" {}    : {}     {}: {}", "Mode".green(), current_mode, "Difficulty".green(), current_difficulty);
    println!(" {}", "Type :help for all meta-commands".yellow());
    println!("{}", "──────────────────────────────────────────".cyan());
    println!();

    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        // Feature 1: Prompt prefix radhe [mode/difficulty] › _
        print!("{} [{}/{}] {} ", 
            "radhe".green().bold(), 
            current_mode.cyan(), 
            current_difficulty.magenta(), 
            "›".yellow().bold()
        );
        io::stdout().flush().context("failed to flush stdout")?;

        let mut input = String::new();
        let bytes_read = reader.read_line(&mut input).context("failed to read from stdin")?;
        
        if bytes_read == 0 {
            println!("\n{}", "Goodbye! Hope Radhe AI helped you today!".cyan().bold());
            break;
        }

        let mut trimmed = input.trim();
        if trimmed.starts_with('\u{feff}') {
            trimmed = trimmed.strip_prefix('\u{feff}').unwrap_or(trimmed);
        }
        if trimmed.is_empty() {
            continue;
        }

        // Add to history
        history.push(trimmed.to_string());

        // Feature 2: Meta-commands with colon prefix
        if trimmed.starts_with(':') {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            let cmd = parts[0];

            match cmd {
                ":mode" => {
                    println!("Current mode: {}", current_mode);
                }
                ":difficulty" => {
                    println!("Current difficulty: {}", current_difficulty);
                }
                ":set-mode" => {
                    if parts.len() < 2 {
                        println!("Usage: :set-mode <normal|exam|revision>");
                    } else {
                        let new_mode = parts[1].to_lowercase();
                        if ["normal", "exam", "revision"].contains(&new_mode.as_str()) {
                            current_mode = new_mode;
                            println!("Session study mode changed to: {}", current_mode);
                        } else {
                            println!("{}: Invalid study mode '{}'. Valid options: normal, exam, revision", "Error".red(), parts[1]);
                        }
                    }
                }
                ":set-difficulty" => {
                    if parts.len() < 2 {
                        println!("Usage: :set-difficulty <easy|medium|hard>");
                    } else {
                        let new_diff = parts[1].to_lowercase();
                        if ["easy", "medium", "hard"].contains(&new_diff.as_str()) {
                            current_difficulty = new_diff;
                            println!("Session quiz difficulty changed to: {}", current_difficulty);
                        } else {
                            println!("{}: Invalid quiz difficulty '{}'. Valid options: easy, medium, hard", "Error".red(), parts[1]);
                        }
                    }
                }
                ":stats" => {
                    if let Some(home) = dirs::home_dir() {
                        let stats_path = home.join(".radhe").join("stats.toml");
                        let stats = load_stats(&stats_path);
                        println!("Radhe AI — Usage Stats");
                        println!("─────────────────────────────");
                        println!("Total commands run : {}", stats.total_commands);
                        println!("Explains           : {}", stats.explain_count);
                        println!("Code generations   : {}", stats.code_count);
                        println!("Quizzes            : {}", stats.quiz_count);
                        println!("Notes              : {}", stats.notes_count);
                        println!("Summaries          : {}", stats.summarize_count);
                        println!("Chats              : {}", stats.chat_count);
                        if !stats.pack_usage.is_empty() {
                            println!("Pack usage:");
                            let mut sorted_packs: Vec<(&String, &u64)> = stats.pack_usage.iter().collect();
                            sorted_packs.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
                            for (pack, count) in sorted_packs {
                                println!("  {:<8}: {}", pack, count);
                            }
                        }
                    } else {
                        println!("Error: Unable to load stats path.");
                    }
                }
                ":clear" => {
                    print!("\x1B[2J\x1B[1;1H");
                    io::stdout().flush().ok();
                }
                ":help" => {
                    println!("{}", "Available Meta-Commands:".cyan().bold());
                    println!("  {:20} — prints current mode", ":mode");
                    println!("  {:20} — prints current difficulty", ":difficulty");
                    println!("  {:20} — changes mode for session (no disk write)", ":set-mode <normal|exam|revision>");
                    println!("  {:20} — changes difficulty for session (no disk write)", ":set-difficulty <easy|medium|hard>");
                    println!("  {:20} — prints the same table as radhe stats", ":stats");
                    println!("  {:20} — clears terminal screen", ":clear");
                    println!("  {:20} — prints last 20 entries numbered", ":history");
                    println!("  {:20} — prints all available meta-commands", ":help");
                    println!("  {:20} — exits the shell", ":quit / :exit");
                }
                ":quit" | ":exit" => {
                    println!("{}", "Goodbye! Hope Radhe AI helped you today!".cyan().bold());
                    break;
                }
                ":history" => {
                    let start = if history.len() > 20 { history.len() - 20 } else { 0 };
                    for (i, entry) in history.iter().skip(start).enumerate() {
                        println!("{}: {}", i + 1, entry);
                    }
                }
                _ => {
                    println!("{}: Unknown meta-command '{}'. Type :help for a list of commands.", "Error".red(), cmd);
                }
            }
            println!();
            continue;
        }

        // Regular inputs: LLM execution
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
                let prompt = match current_mode.as_str() {
                    "exam" => format!(
                        "Explain '{topic}' for a beginner programmer. Provide very short, direct, exam-focused answers with zero fluff. Stop after 3 sentences.{lang_suffix}"
                    ),
                    "revision" => format!(
                        "Explain '{topic}' for a beginner programmer using bullet-style concise memory aids and quick revision facts. Keep them extremely short and punchy.{lang_suffix}"
                    ),
                    _ => format!(
                        "Explain '{topic}' in exactly 5 bullet points for a beginner programmer. Each bullet must be one sentence. Stop after 5 bullets. Do not repeat yourself.\n\nExplanation:{lang_suffix}"
                    ),
                };
                (prompt, "explain", 200)
            } else if trimmed.starts_with("--notes ") {
                let topic = trimmed["--notes ".len()..].trim();
                let prompt = match current_mode.as_str() {
                    "exam" => format!(
                        "Give exactly 6 bullet points about '{topic}' for a student. Focus strictly on exam-relevant facts, direct and with zero fluff. Format strictly as:
- [fact 1]
- [fact 2]
- [fact 3]
- [fact 4]
- [fact 5]
- [fact 6]
Each bullet = one unique fact. Max 15 words per bullet. Start directly with the first bullet, no intro paragraph.{lang_suffix}"
                    ),
                    "revision" => format!(
                        "Give exactly 6 bullet points about '{topic}' for a student. Use bullet-style concise memory aids and quick revision facts that are extremely punchy. Format strictly as:
- [fact 1]
- [fact 2]
- [fact 3]
- [fact 4]
- [fact 5]
- [fact 6]
Each bullet = one unique fact. Max 15 words per bullet. Start directly with the first bullet, no intro paragraph.{lang_suffix}"
                    ),
                    _ => format!(
                        "Give exactly 6 bullet points about '{topic}' for a student. Format strictly as:
- [fact 1]
- [fact 2]
- [fact 3]
- [fact 4]
- [fact 5]
- [fact 6]
Each bullet = one unique fact. Max 15 words per bullet. Start directly with the first bullet, no intro paragraph.{lang_suffix}"
                    ),
                };
                (prompt, "notes", 150)
            } else {
                let prompt = match current_mode.as_str() {
                    "exam" => format!(
                        "Explain '{trimmed}' for a beginner programmer. Provide very short, direct, exam-focused answers with zero fluff. Stop after 3 sentences.{lang_suffix}"
                    ),
                    "revision" => format!(
                        "Explain '{trimmed}' for a beginner programmer using bullet-style concise memory aids and quick revision facts. Keep them extremely short and punchy.{lang_suffix}"
                    ),
                    _ => format!(
                        "Explain '{trimmed}' in exactly 5 bullet points for a beginner programmer. Each bullet must be one sentence. Stop after 5 bullets. Do not repeat yourself.\n\nExplanation:{lang_suffix}"
                    ),
                };
                (prompt, "explain", 200)
            }
        };

        // Call run_inference
        match run_inference(&prompt_text, model, max_tokens, mode) {
            Ok(output) => {
                if let Some(home) = dirs::home_dir() {
                    let stats_path = home.join(".radhe").join("stats.toml");
                    let mut stats = load_stats(&stats_path);
                    stats.total_commands += 1;
                    match mode {
                        "code" => stats.code_count += 1,
                        "explain" => stats.explain_count += 1,
                        "notes" => stats.notes_count += 1,
                        _ => {}
                    }
                    let _ = save_stats(&stats_path, &stats);
                }
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

fn run_chat(active_model: &str, active_lang: &str, study_mode: &str) -> Result<()> {
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
        let system_instruction = match study_mode {
            "exam" => format!("You are Radhe, a concise AI assistant for students in EXAM mode. Give extremely short, direct answers with zero fluff. Answer questions directly without headers. Maximum 2 short sentences per response.{}", lang_instruction),
            "revision" => format!("You are Radhe, a concise AI assistant for students in REVISION mode. Give answers using bullet-style concise memory aids. Keep them extremely brief and punchy.{}", lang_instruction),
            _ => format!("You are Radhe, a concise AI assistant for students. Give short, direct answers. No bullet points unless asked. No headers. Maximum 3 sentences per response.{}", lang_instruction),
        };
        let mut prompt = format!("<|im_start|>system\n{}<|im_end|>\n", system_instruction);
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
}#[derive(serde::Serialize, serde::Deserialize, Default, Debug)]
struct RadheStats {
    #[serde(default)]
    total_commands: u64,
    #[serde(default)]
    explain_count: u64,
    #[serde(default)]
    code_count: u64,
    #[serde(default)]
    quiz_count: u64,
    #[serde(default)]
    notes_count: u64,
    #[serde(default)]
    summarize_count: u64,
    #[serde(default)]
    chat_count: u64,
    #[serde(default)]
    pack_usage: std::collections::BTreeMap<String, u64>,
}

fn load_stats(path: &std::path::Path) -> RadheStats {
    if let Some(home) = dirs::home_dir() {
        let config_path = home.join(".radhe").join("config.toml");
        if config_path.exists() {
            if let Ok(content) = fs::read_to_string(&config_path) {
                if let Ok(cfg) = toml::from_str::<RadheConfig>(&content) {
                    if cfg.stats_enabled == Some(false) {
                        return RadheStats::default();
                    }
                }
            }
        }
    }
    if path.exists() {
        let content = fs::read_to_string(path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    } else {
        RadheStats::default()
    }
}

fn save_stats(path: &std::path::Path, stats: &RadheStats) -> Result<()> {
    if let Some(home) = dirs::home_dir() {
        let config_path = home.join(".radhe").join("config.toml");
        if config_path.exists() {
            if let Ok(content) = fs::read_to_string(&config_path) {
                if let Ok(cfg) = toml::from_str::<RadheConfig>(&content) {
                    if cfg.stats_enabled == Some(false) {
                        return Ok(());
                    }
                }
            }
        }
    }
    let content = toml::to_string(stats)?;
    fs::write(path, content)?;
    Ok(())
}

fn run_setup(config_path: &std::path::Path, current_mode: &str, current_difficulty: &str, current_lang: &str) -> Result<()> {
    use std::io::{self, Write};
    use colored::Colorize;

    // Show profile picker
    println!("{}", "╔══════════════════════════════════════════╗".cyan());
    println!("{}", "║        Radhe AI — First-Run Setup        ║".cyan());
    println!("{}", "╚══════════════════════════════════════════╝".cyan());
    println!("Choose your setup profile:");
    println!("  1) {}  — Just ask questions, get answers.", "Minimal".yellow());
    println!("                No stats, no shell, no packs. Fastest.");
    println!("  2) {} — Recommended. Stats tracking, radhe shell,", "Standard".yellow());
    println!("                study modes, difficulty. Everything useful.");
    println!("  3) {}   — Choose exactly which features to enable.", "Custom".yellow());

    let profile_choice;
    loop {
        print!("Select profile [1-3]: ");
        io::stdout().flush().context("failed to flush stdout")?;
        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input).context("failed to read from stdin")?;
        if bytes_read == 0 {
            println!("Setup aborted (EOF).");
            return Ok(());
        }
        let trimmed = input.trim();
        if trimmed == "1" || trimmed == "2" || trimmed == "3" {
            profile_choice = trimmed.to_string();
            break;
        } else {
            println!("Invalid selection. Please choose 1, 2, or 3.");
        }
    }

    // Load existing config
    let content = if config_path.exists() {
        std::fs::read_to_string(config_path).unwrap_or_default()
    } else {
        String::new()
    };
    let mut config: RadheConfig = toml::from_str(&content).unwrap_or_default();

    if profile_choice == "1" {
        // Profile 1 — Minimal
        config.model = Some(config.model.clone().unwrap_or_else(|| "Qwen2.5-Coder-1.5B-Instruct-Q4_K_M.gguf".to_string()));
        config.max_tokens = Some(config.max_tokens.unwrap_or(300));
        config.lang = Some("en".to_string());
        config.mode = Some("normal".to_string());
        config.difficulty = Some("medium".to_string());
        config.profile = Some("minimal".to_string());
        config.stats_enabled = Some(false);
        config.packs_enabled = Some(false);
        config.shell_enabled = Some(false);
        config.modes_enabled = Some(false);
        config.difficulty_enabled = Some(false);

        let updated_content = toml::to_string(&config)?;
        std::fs::write(config_path, updated_content)?;

        println!("{}", "✓ Minimal profile set. Run: radhe \"your question\"".green());
        return Ok(());
    }

    let modes_enabled;
    let difficulty_enabled;

    if profile_choice == "2" {
        modes_enabled = true;
        difficulty_enabled = true;

        config.profile = Some("standard".to_string());
        config.stats_enabled = Some(true);
        config.packs_enabled = Some(true);
        config.shell_enabled = Some(true);
        config.modes_enabled = Some(true);
        config.difficulty_enabled = Some(true);
    } else {
        // Profile 3 — Custom
        let ask_feature = |prompt: &str| -> bool {
            loop {
                print!("{} [Y/n]: ", prompt);
                io::stdout().flush().ok();
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Err(_) => return true,
                    Ok(0) => return true,
                    Ok(_) => {}
                }
                let trimmed = input.trim().to_lowercase();
                if trimmed.is_empty() || trimmed == "y" || trimmed == "yes" {
                    return true;
                } else if trimmed == "n" || trimmed == "no" {
                    return false;
                } else {
                    println!("Invalid input. Enter Y/y or N/n, or just press Enter.");
                }
            }
        };

        let stats_val = ask_feature("Enable usage stats (radhe stats)?");
        let packs_val = ask_feature("Enable subject packs (radhe --pack)?");
        let shell_val = ask_feature("Enable power shell (radhe shell)?");
        modes_enabled = ask_feature("Enable study modes (--mode exam/revision)?");
        difficulty_enabled = ask_feature("Enable quiz difficulty (--difficulty hard)?");

        config.profile = Some("custom".to_string());
        config.stats_enabled = Some(stats_val);
        config.packs_enabled = Some(packs_val);
        config.shell_enabled = Some(shell_val);
        config.modes_enabled = Some(modes_enabled);
        config.difficulty_enabled = Some(difficulty_enabled);
    }

    // Now run the mode/difficulty/language menus only for features that were enabled.
    let mut selected_mode = current_mode.to_string();
    if modes_enabled {
        // 1. Study Mode Selector
        println!("{}", "┌─ Study Mode ────────────────────────────┐".cyan());
        println!("│  1) {}   — full explanations        │", "normal".yellow());
        println!("│  2) {}     — short, direct answers    │", "exam".yellow());
        println!("│  3) {} — bullet memory aids       │", "revision".yellow());
        println!("{}", "└─────────────────────────────────────────┘".cyan());

        loop {
            print!("Select mode [1-3] (current: {}): ", current_mode);
            io::stdout().flush().context("failed to flush stdout")?;
            let mut input = String::new();
            let bytes_read = io::stdin().read_line(&mut input).context("failed to read from stdin")?;
            if bytes_read == 0 {
                break;
            }
            let trimmed = input.trim();
            if trimmed.is_empty() {
                break;
            }
            match trimmed {
                "1" => { selected_mode = "normal".to_string(); break; }
                "2" => { selected_mode = "exam".to_string(); break; }
                "3" => { selected_mode = "revision".to_string(); break; }
                _ => println!("Invalid selection. Please choose 1, 2, or 3, or press Enter to keep current."),
            }
        }
    }

    let mut selected_diff = current_difficulty.to_string();
    if difficulty_enabled {
        // 2. Quiz Difficulty Selector
        println!("{}", "┌─ Quiz Difficulty ───────────────────────┐".cyan());
        println!("│  1) {}    — basic recall              │", "easy".yellow());
        println!("│  2) {}  — conceptual understanding  │", "medium".yellow());
        println!("│  3) {}    — critical thinking         │", "hard".yellow());
        println!("{}", "└─────────────────────────────────────────┘".cyan());

        loop {
            print!("Select difficulty [1-3] (current: {}): ", current_difficulty);
            io::stdout().flush().context("failed to flush stdout")?;
            let mut input = String::new();
            let bytes_read = io::stdin().read_line(&mut input).context("failed to read from stdin")?;
            if bytes_read == 0 {
                break;
            }
            let trimmed = input.trim();
            if trimmed.is_empty() {
                break;
            }
            match trimmed {
                "1" => { selected_diff = "easy".to_string(); break; }
                "2" => { selected_diff = "medium".to_string(); break; }
                "3" => { selected_diff = "hard".to_string(); break; }
                _ => println!("Invalid selection. Please choose 1, 2, or 3, or press Enter to keep current."),
            }
        }
    }

    // 3. Language Selector
    println!("{}", "┌─ Language ──────────────────────────────┐".cyan());
    println!("│  1) {}       — English                  │", "en".yellow());
    println!("│  2) {}       — Hindi (Devanagari)       │", "hi".yellow());
    println!("│  3) {} — Hindi + English mix      │", "hinglish".yellow());
    println!("{}", "└─────────────────────────────────────────┘".cyan());

    let mut selected_lang = current_lang.to_string();
    loop {
        print!("Select language [1-3] (current: {}): ", current_lang);
        io::stdout().flush().context("failed to flush stdout")?;
        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input).context("failed to read from stdin")?;
        if bytes_read == 0 {
            break;
        }
        let trimmed = input.trim();
        if trimmed.is_empty() {
            break;
        }
        match trimmed {
            "1" => { selected_lang = "en".to_string(); break; }
            "2" => { selected_lang = "hi".to_string(); break; }
            "3" => { selected_lang = "hinglish".to_string(); break; }
            _ => println!("Invalid selection. Please choose 1, 2, or 3, or press Enter to keep current."),
        }
    }

    config.mode = Some(selected_mode);
    config.difficulty = Some(selected_diff);
    config.lang = Some(selected_lang);

    let updated_content = toml::to_string(&config)?;
    std::fs::write(config_path, updated_content)?;

    if profile_choice == "2" {
        println!("{}", "✓ Standard profile active. All features enabled.".green());
    } else {
        println!("{}", "✓ Custom profile active.".green());
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
            mode: None,
            difficulty: None,
            set_mode: None,
            set_difficulty: None,
        };
        let p = build_prompt(&cli, "en", "medium", "normal").unwrap();
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
            mode: None,
            difficulty: None,
            set_mode: None,
            set_difficulty: None,
        };
        let p = build_prompt(&cli, "en", "medium", "normal").unwrap();
        assert!(p.contains("Explain 'recursion' in exactly 5 bullet points"), "should format explanation prompt");
    }
}




