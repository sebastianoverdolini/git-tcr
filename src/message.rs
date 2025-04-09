use std::io::Write;
use std::process::{Command, Stdio};

// TODO Test
// TODO Manage fallbacks better
// TODO Speak more: highlight the generation of message through AI or not.

const PROMPT: &str = "\
Given the following git diff, write a commit message.\
Do not output anything else than the commit message.\
The commit message must be in the imperative form.\
The commit message must be max 72 characters long.\
The commit message must be a unique sentence.\
The first letter of the commit message must be capitalized.";

pub fn message() -> String {
    let prompt = format!("{PROMPT}\n{}", get_diff());

    match generate_ai_message(&prompt) {
        Some(msg) if !msg.is_empty() => {
            println!("AI MESSAGE");
            msg
        }
        Some(_) => {
            println!("EMPTY AI MESSAGE, DEFAULT WIP");
            "WIP".to_string()
        }
        None => {
            println!("AI MESSAGE DIDN'T WORK, DEFAULT WIP");
            "WIP".to_string()
        }
    }
}

fn generate_ai_message(prompt: &str) -> Option<String> {
    let mut child = Command::new("ollama")
        .args(["run", "mistral"])
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
        .args(["diff"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default()
}
