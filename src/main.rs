use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::{
    fs,
    io::ErrorKind,
    path::PathBuf,
    process::{Command, Stdio},
};

#[derive(Parser, Debug)]
#[command(name = "radhe")]
#[command(version, about = "Tiny offline terminal AI assistant for students")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

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

    #[arg(long, default_value = "qwen2")]
    model: String,

    #[arg(long, default_value_t = 256)]
    max_tokens: u32,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init,
    Doctor,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init) => {
            println!("Initializing Radhe AI directories...");
            init_dirs()?;
            println!("Done.");
            return Ok(());
        }
        Some(Commands::Doctor) => {
            run_doctor();
            return Ok(());
        }
        None => {}
    }

    let is_repl = cli.prompt.is_none()
        && cli.code.is_none()
        && cli.explain.is_none()
        && cli.notes.is_none()
        && cli.fix.is_none();

    if is_repl {
        run_repl(&cli.model)?;
        return Ok(());
    }

    let prompt = build_prompt(&cli)?;
    let mode = if cli.code.is_some() {
        "code"
    } else if cli.explain.is_some() {
        "explain"
    } else if cli.notes.is_some() {
        "notes"
    } else if cli.fix.is_some() {
        "fix"
    } else {
        "prompt"
    };
    let max_tokens = match mode {
        "code" => 300,
        "explain" => 200,
        "notes" => 150,
        "fix" => 400,
        _ => cli.max_tokens,
    };
    let output = run_inference(&prompt, &cli.model, max_tokens, mode)
        .context("failed to run local inference")?;

    println!("{output}");
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

    if let Some(prompt) = &cli.prompt {
        return Ok(format!(
            "You are Radhe AI, a tiny offline terminal assistant for students. Be concise and practical.
User: {prompt}"
        ));
    }

    anyhow::bail!("no prompt provided. Try: radhe --code \"bubble sort in c\"")
}

fn run_inference(prompt: &str, model: &str, max_tokens: u32, mode: &str) -> Result<String> {
    let model_path = dirs::home_dir()
        .context("unable to find user home directory")?
        .join(".radhe")
        .join("models")
        .join(format!("{model}.gguf"));
    let model_path = model_path.to_string_lossy().into_owned();

    let prompt_with_delim = if mode == "fix" {
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
        let end_pos = cleaned_content.find(&prompt_normalized)
            .map(|p| p + prompt_normalized.len())
            .or_else(|| cleaned_content.find(&trimmed_prompt_normalized).map(|p| p + trimmed_prompt_normalized.len()))
            .unwrap_or(0);

        let rest = &cleaned_content[end_pos..];
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

fn run_doctor() {
    println!("- Checking llama-completion.exe in PATH...");
    match Command::new("llama-completion.exe").arg("--help").output() {
        Ok(_) => println!("  OK: llama-completion.exe found"),
        Err(_) => println!("  MISSING: llama-completion.exe not found"),
    }

    let model_path = dirs::home_dir()
        .map(|p| p.join(".radhe").join("models").join("qwen2.gguf"));

    if let Some(path) = &model_path {
        println!("- Expected model path: {}", path.display());
        if path.exists() {
            println!("  OK: model found");
        } else {
            println!("  MISSING: download model to ~/.radhe/models/qwen2.gguf");
        }
    } else {
        println!("- Expected model path: unable to resolve home directory");
        println!("  MISSING: download model to ~/.radhe/models/qwen2.gguf");
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
