use std::fs;
use std::process::Command;

pub fn wip(_diff: &str) -> String {
    "WIP".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wip_returns_wip() {
        assert_eq!(wip("ignored"), "WIP");
    }
}

pub fn auto(diff: &str) -> String {
    let prompt_path = "src/commit.md";
    let prompt = fs::read_to_string(prompt_path).unwrap_or_else(|_| "Generate a Git commit message from the diff below.".to_string());
    let full_prompt = format!("{}\nGit diff:\n{}", prompt, diff);
    let output = Command::new("ollama")
        .arg("run")
        .arg("qwen2.5-coder:3b")
        .arg(full_prompt)
        .output();
    match output {
        Ok(output) if output.status.success() => {
            let msg = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if msg.is_empty() {
                "[LLM returned empty message]".to_string()
            } else {
                msg
            }
        }
        _ => "[LLM failed to generate message]".to_string(),
    }
}
