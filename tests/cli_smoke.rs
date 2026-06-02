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

#[test]
fn test_stats_subcommand() {
    let output = radhe().arg("stats").output().expect("failed to run radhe");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Total commands run"), "stats should print usage stats");
    assert!(output.status.success());
}

#[test]
fn test_stats_reset_no() {
    use std::io::Write;
    let mut child = radhe()
        .args(["stats", "--reset"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn radhe");

    let mut stdin = child.stdin.take().expect("failed to open stdin");
    stdin.write_all(b"n\n").expect("failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("failed to read output");
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!combined.contains("Statistics wiped"), "reset should not happen if user enters 'n'");
    assert!(output.status.success());
}

#[test]
fn test_shell_help_flag() {
    let output = radhe().args(["shell", "--help"]).output().expect("failed to run radhe");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.to_lowercase().contains("shell"), "shell help should contain 'shell'");
    assert!(output.status.success());
}

#[test]
fn test_set_mode_invalid() {
    let output = radhe().args(["--set-mode", "invalid"]).output().expect("failed to run radhe");
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(combined.to_lowercase().contains("invalid") || !output.status.success(), "should print error or return non-zero code");
}

#[test]
fn test_set_difficulty_invalid() {
    let output = radhe().args(["--set-difficulty", "invalid"]).output().expect("failed to run radhe");
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(combined.to_lowercase().contains("invalid") || !output.status.success(), "should print error or return non-zero code");
}

#[test]
fn test_set_mode_valid() {
    let output = radhe().args(["--set-mode", "normal"]).output().expect("failed to run radhe");
    assert!(output.status.success(), "set-mode normal should exit with 0");
}

#[test]
fn test_set_difficulty_valid() {
    let output = radhe().args(["--set-difficulty", "medium"]).output().expect("failed to run radhe");
    assert!(output.status.success(), "set-difficulty medium should exit with 0");
}

#[test]
fn test_doctor_shows_mode() {
    let output = radhe().arg("doctor").output().expect("failed to run radhe");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Study Mode"), "doctor should print 'Study Mode'");
    assert!(output.status.success());
}

#[test]
fn test_doctor_shows_difficulty() {
    let output = radhe().arg("doctor").output().expect("failed to run radhe");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Quiz Difficulty"), "doctor should print 'Quiz Difficulty'");
    assert!(output.status.success());
}

#[test]
fn test_doctor_shows_shell() {
    let output = radhe().arg("doctor").output().expect("failed to run radhe");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Shell"), "doctor should print 'Shell'");
    assert!(output.status.success());
}
