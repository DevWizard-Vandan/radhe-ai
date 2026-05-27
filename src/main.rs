use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::{
    fs,
    io::ErrorKind,
    path::PathBuf,
    process::{Command, Stdio},
};

#[derive(Deserialize, Default, Debug)]
struct RadheConfig {
    model: Option<String>,
    max_tokens: Option<u32>,
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

    if let Some(ref path) = cli.summarize {
        let abs_path = std::fs::canonicalize(path)
            .unwrap_or_else(|_| std::path::PathBuf::from(path));
        cli.summarize = Some(abs_path.to_string_lossy().into_owned());
    }

    if let Some(ref path) = cli.fix {
        let abs_path = std::fs::canonicalize(path)
            .unwrap_or_else(|_| std::path::PathBuf::from(path));
        cli.fix = Some(abs_path.to_string_lossy().into_owned());
    }

    if let Some(ref path) = cli.quiz_file {
        let abs_path = std::fs::canonicalize(path)
            .unwrap_or_else(|_| std::path::PathBuf::from(path));
        cli.quiz_file = Some(abs_path.to_string_lossy().into_owned());
    }

    // Keep 0.5B accessible via --model qwen-0.5b override flag
    let active_model = cli.model
        .clone()
        .or_else(|| config.model.clone())
        .unwrap_or_else(|| "Qwen2.5-Coder-1.5B-Instruct-Q4_K_M.gguf".to_string());

    let active_max_tokens = cli.max_tokens
        .or(config.max_tokens)
        .unwrap_or(300);

    match cli.command {
        Some(Commands::Init) => {
            println!("Initializing Radhe AI directories...");
            init_dirs()?;
            println!("Done.");
            return Ok(());
        }
        Some(Commands::Doctor) => {
            run_doctor(&active_model);
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
        run_repl(&active_model)?;
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
        run_chat(&active_model)?;
        return Ok(());
    }

    let prompt = build_prompt(&cli)?;
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

fn build_prompt(cli: &Cli) -> Result<String> {
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
            "You are a coding assistant. Return ONLY valid compilable code with zero explanation. No markdown, no backticks, no comments. Just raw code.
Task: {task_str}"
        ));
    }

    if let Some(topic) = &cli.explain {
        return Ok(format!(
            "Explain '{topic}' in exactly 5 bullet points for a beginner programmer. Each bullet must be one sentence. Stop after 5 bullets. Do not repeat yourself.\n\nExplanation:"
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
Each bullet = one unique fact. Max 15 words per bullet. Start directly with the first bullet, no intro paragraph."
        ));
    }

    if let Some(file_path_str) = &cli.fix {
        let path = PathBuf::from(file_path_str);
        if !path.exists() {
            anyhow::bail!("File not found: {file_path_str}. Please provide a valid file path.");
        }
        let code = fs::read_to_string(&path)
            .with_context(|| format!("unable to read file: {file_path_str}"))?;

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

    if let Some(file_path_str) = &cli.summarize {
        let path = PathBuf::from(file_path_str);
        if !path.exists() {
            eprintln!("Error: Could not read file '{}'. Does it exist?", file_path_str);
            std::process::exit(1);
        }
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Error: Could not read file '{}'. Does it exist?", file_path_str);
                std::process::exit(1);
            }
        };
        let trimmed_content = content.trim();
        if trimmed_content.is_empty() {
            eprintln!("Error: File is empty.");
            std::process::exit(1);
        }
        let truncated: String = trimmed_content.chars().take(3000).collect();

        return Ok(format!(
            "You are a study assistant. Summarize the following notes into exactly 5 clear bullet points. Each bullet should be one concise sentence. Start each bullet with a dash (-).

Notes:
{}",
            truncated
        ));
    }

    if let Some(file_path_str) = &cli.quiz_file {
        let path = PathBuf::from(file_path_str);
        if !path.exists() {
            eprintln!("Error: Could not read file '{}'. Does it exist?", file_path_str);
            std::process::exit(1);
        }
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Error: Could not read file '{}'. Does it exist?", file_path_str);
                std::process::exit(1);
            }
        };
        let trimmed_content = content.trim();
        if trimmed_content.is_empty() {
            eprintln!("Error: File is empty.");
            std::process::exit(1);
        }
        let truncated: String = trimmed_content.chars().take(3000).collect();

        return Ok(format!(
            "You are a student quiz generator. Based on the following notes, generate exactly 5 quiz questions with answers. Format each as:
Q1: [question]
A1: [answer]
Q2: [question]
A2: [answer]
... and so on until Q5/A5.

Notes:
{}",
            truncated
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
Only output the questions. No intro, no explanation."
        ));
    }

    if let Some(prompt) = &cli.prompt {
        return Ok(format!(
            "You are Radhe AI, a tiny offline terminal assistant for students. Be concise and practical.
User: {prompt}"
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
    let model_path = dirs::home_dir()
        .context("unable to find user home directory")?
        .join(".radhe")
        .join("models")
        .join(&model_filename);
    let model_path = model_path.to_string_lossy().into_owned();

    let prompt_with_delim = if mode == "fix" || mode == "chat" {
        prompt.to_string()
    } else {
        format!("{}\n\n### RESPONSE:\n", prompt)
    };

    let max_tokens_str = max_tokens.to_string();
    let child = Command::new("llama-completion.exe")
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

fn run_doctor(active_model: &str) {
    use colored::Colorize;

    let version = env!("CARGO_PKG_VERSION");
    println!("Radhe AI v{}", version);
    println!("Running diagnostics...");

    let mut all_ok = true;

    // 1. Check llama-completion.exe in PATH
    match Command::new("llama-completion.exe").arg("--help").output() {
        Ok(_) => {
            println!("{}", "✓ llama-completion.exe found".green());
        }
        Err(_) => {
            println!("{}", "✗ llama-completion.exe not found".red());
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

    // 3. Model warning if not default 1.5B
    if active_model != "Qwen2.5-Coder-1.5B-Instruct-Q4_K_M.gguf" && active_model != "Qwen2.5-Coder-1.5B-Instruct-Q4_K_M" {
        println!("{}", format!("! Warning: Using non-default model ({})", active_model).yellow());
    }

    // 4. Print final status
    if all_ok {
        println!("{}", "All systems operational.".green());
    } else {
        println!("{}", "✗ Systems check failed. Check the errors above.".red());
    }
}

fn run_repl(model: &str) -> Result<()> {
    use colored::Colorize;
    use std::io::{self, BufRead, Write};

    // Set up Ctrl+C handler with a friendly cyan/yellow exit message
    ctrlc::set_handler(move || {
        println!("\n\n{}", "Goodbye! Hope Radhe AI helped you today!".cyan().bold());
        std::process::exit(0);
    })
    .context("Error setting Ctrl-C handler")?;

    // Print welcome header
    println!("{}", "Radhe AI v0.1.0 — Offline Terminal Assistant".cyan().bold());
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
        } else if trimmed.starts_with("--explain ") {
            let topic = trimmed["--explain ".len()..].trim();
            let prompt = format!(
                "Explain '{topic}' in exactly 5 bullet points for a beginner programmer. Each bullet must be one sentence. Stop after 5 bullets. Do not repeat yourself.\n\nExplanation:"
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
Each bullet = one unique fact. Max 15 words per bullet. Start directly with the first bullet, no intro paragraph."
            );
            (prompt, "notes", 150)
        } else {
            let prompt = format!(
                "Explain '{trimmed}' in exactly 5 bullet points for a beginner programmer. Each bullet must be one sentence. Stop after 5 bullets. Do not repeat yourself.\n\nExplanation:"
            );
            (prompt, "explain", 200)
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

    let ps_output = Command::new("powershell")
        .args([
            "-Command",
            "Invoke-RestMethod -Uri 'https://api.github.com/repos/DevWizard-Vandan/radhe-ai/releases/latest' | Select-Object -ExpandProperty tag_name"
        ])
        .output();

    let output = match ps_output {
        Ok(out) => out,
        Err(_) => {
            anyhow::bail!("Update check failed. Are you connected to the internet?");
        }
    };

    if !output.status.success() {
        anyhow::bail!("Update check failed. Are you connected to the internet?");
    }

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let mut latest_version = stdout_str.trim().to_string();
    if latest_version.starts_with('v') {
        latest_version = latest_version[1..].to_string();
    }

    if latest_version.is_empty() {
        anyhow::bail!("Update check failed. Are you connected to the internet?");
    }

    if current_version == latest_version {
        println!("Radhe AI is already up to date (v{current_version})");
        return Ok(());
    }

    println!("Update available: v{current_version} → v{latest_version}");
    println!("Downloading new binary...");

    let current_exe = std::env::current_exe()?;
    let exe_dir = current_exe.parent().context("failed to get current exe directory")?;
    let new_exe_path = exe_dir.join("radhe_new.exe");
    let new_exe_path_str = new_exe_path.to_string_lossy().to_string();

    let download_url = "https://github.com/DevWizard-Vandan/radhe-ai/releases/latest/download/radhe.exe";
    let download_status = Command::new("powershell")
        .args([
            "-Command",
            &format!("Invoke-WebRequest -Uri '{}' -OutFile '{}'", download_url, new_exe_path_str)
        ])
        .output();

    let download_success = match download_status {
        Ok(out) => out.status.success(),
        Err(_) => false,
    };

    if !download_success {
        anyhow::bail!("Download failed. Please try again or update manually.");
    }

    // Replace the current binary
    let old_exe_path = exe_dir.join("radhe_old.exe");
    if old_exe_path.exists() {
        let _ = fs::remove_file(&old_exe_path);
    }

    fs::rename(&current_exe, &old_exe_path).context("failed to rename current binary to radhe_old.exe")?;
    fs::rename(&new_exe_path, &current_exe).context("failed to rename radhe_new.exe to radhe.exe")?;

    println!("Radhe AI updated to v{latest_version}! Restart your terminal.");
    Ok(())
}

fn run_chat(active_model: &str) -> Result<()> {
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

        // Build rolling prompt from history
        let mut prompt = String::from("<|im_start|>system\nYou are Radhe, a concise AI assistant for students. Give short, direct answers. No bullet points unless asked. No headers. Maximum 3 sentences per response.<|im_end|>\n");
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
        };
        let p = build_prompt(&cli).unwrap();
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
        };
        let p = build_prompt(&cli).unwrap();
        assert!(p.contains("Explain 'recursion' in exactly 5 bullet points"), "should format explanation prompt");
    }
}




