use std::process::Command;

fn radhe() -> Command {
    Command::new(env!("CARGO_BIN_EXE_radhe"))
}

#[test]
fn test_version_flag() {
    let output = radhe().arg("--version").output().expect("failed to run radhe");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Radhe AI v"), "--version should print version");
    assert!(stdout.contains("Model:"), "--version should print model name");
    assert!(output.status.success());
}

#[test]
fn test_doctor_subcommand() {
    let output = radhe().arg("doctor").output().expect("failed to run radhe");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Radhe AI v"), "doctor should print version header");
    assert!(stdout.contains("diagnostics"), "doctor should mention diagnostics");
    assert!(stdout.contains("Shell: available"), "doctor should print shell availability status");
    assert!(output.status.success());
}

#[test]
fn test_models_subcommand() {
    let output = radhe().arg("models").output().expect("failed to run radhe");
    assert!(output.status.success(), "models subcommand should exit 0");
}

#[test]
fn test_summarize_missing_file() {
    let output = radhe()
        .args(["--summarize", "nonexistent_file_xyz.txt"])
        .output()
        .expect("failed to run radhe");
    assert!(!output.status.success(), "should fail on missing file");
    let stderr_or_stdout = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stderr_or_stdout.contains("Could not read file"), "should print friendly error");
}

#[test]
fn test_quiz_file_missing_file() {
    let output = radhe()
        .args(["--quiz-file", "nonexistent_file_xyz.txt"])
        .output()
        .expect("failed to run radhe");
    assert!(!output.status.success(), "should fail on missing file");
}
