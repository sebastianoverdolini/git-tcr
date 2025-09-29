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

const PROMPT: &str = r#"You are an intelligent assistant specialized in generating Git commit messages.
You will receive a code diff as input.

Your commit messages must be:
- Short and concise, preferably 3–10 words.
- Direct and clear, starting with a verb (e.g., Refactor, Update, Remove, Add, Start, Integrate, Cleanup, Improve).
- Focused on what changes in the code, not on unnecessary technical details.
- Consistent with the style of existing commits.

Examples of commit messages:
- Inline config in main
- Use program also for test and remove sh dependency
- Refactor
- Keep refactoring towards specific cmd
- Start using specific cmd to run tcr
- Remove --watch
- Run TCR instead of returning the cmd
- Refactor function to accept program and args; update test cases accordingly
- Update AI model to use qwen2.5-coder:14b for better message generation
- Integrate AI powered automatic message

Avoid:
- Long or overly descriptive sentences
- Personal comments or extra information
- Periods at the end of the message
- Double quotes

Generate **a single concise commit message** based on the diff provided below."#;


pub fn scribe(diff: &str) -> String {
    let full_prompt = format!("{PROMPT}\nGit diff:\n{diff}");
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
