use std::io::Write;
use std::process::{Command, Stdio};

// TODO Test
// TODO Manage fallbacks better
// TODO Speak more: highlight the generation of message through AI or not.
// TODO Improve prompt
// TODO Diff with previous commit

const PROMPT: &str = include_str!("message_prompt.txt");

pub fn message() -> String {
    let prompt = format!("{PROMPT}\n{}", get_diff());

    let msg = match generate_ai_message(&prompt) {
        Some(msg) if !msg.is_empty() => msg,
        Some(_) => "WIP".to_string(),
        None => "WIP".to_string()

    };

    msg.trim_matches('"').to_string()
}

fn generate_ai_message(prompt: &str) -> Option<String> {
    let mut child = Command::new("ollama")
        .args(["run", "qwen2.5-coder:14b"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .ok()?;

    child.stdin.as_mut()?.write_all(prompt.as_bytes()).ok()?;

    let output = child.wait_with_output().ok()?;
    String::from_utf8(output.stdout).ok().map(|s| s.trim().to_string())
}


fn get_diff() -> String {
    Command::new("git")
        .args(["diff", "HEAD", "--patience", "-U15"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default()
}
