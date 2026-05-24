use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf, process::Command};

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
    fix: Option<PathBuf>,

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

    let prompt = build_prompt(&cli)?;
    let output = run_inference(&prompt, &cli.model, cli.max_tokens)
        .context("failed to run local inference")?;

    println!("{output}");
    Ok(())
}

fn build_prompt(cli: &Cli) -> Result<String> {
    if let Some(task) = &cli.code {
        return Ok(format!(
            "You are a concise coding assistant. Return ONLY valid compilable code with no explanation.
Task: {task}"
        ));
    }

    if let Some(topic) = &cli.explain {
        return Ok(format!(
            "Explain this simply for a beginner programmer in 4-6 short lines.
Topic: {topic}"
        ));
    }

    if let Some(topic) = &cli.notes {
        return Ok(format!(
            "Write short student-friendly notes in bullet points. Keep it compact and useful.
Topic: {topic}"
        ));
    }

    if let Some(file) = &cli.fix {
        let code = fs::read_to_string(file)
            .with_context(|| format!("unable to read file: {}", file.display()))?;
        return Ok(format!(
            "Fix the code below. Return ONLY the corrected code.

{code}"
        ));
    }

    if let Some(prompt) = &cli.prompt {
        return Ok(format!(
            "You are Radhe AI, a tiny offline terminal assistant for students. Be concise and practical.
User: {prompt}"
        ));
    }

    anyhow::bail!("no prompt provided. Try: radhe --code "bubble sort in c"")
}

fn run_inference(prompt: &str, model: &str, max_tokens: u32) -> Result<String> {
    let model_path = format!("./models/{model}.gguf");
    let output = Command::new("llama-cli")
        .args([
            "-m",
            &model_path,
            "-p",
            prompt,
            "-n",
            &max_tokens.to_string(),
        ])
        .output()
        .context("llama-cli not found in PATH. Install llama.cpp first.")?;

    if !output.status.success() {
        anyhow::bail!(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn init_dirs() -> Result<()> {
    fs::create_dir_all("models")?;
    fs::create_dir_all("installer")?;
    fs::create_dir_all(".radhe")?;
    Ok(())
}

fn run_doctor() {
    println!("- Checking llama-cli in PATH...");
    match Command::new("llama-cli").arg("--help").output() {
        Ok(_) => println!("  OK: llama-cli found"),
        Err(_) => println!("  MISSING: llama-cli not found"),
    }

    println!("- Expected model path: ./models/qwen2.gguf");
}
