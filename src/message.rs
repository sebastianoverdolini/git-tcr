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
The commit message must be a unique sentence. \
The first letter of the commit message must be capitalized.\
";

pub fn message() -> String {
    let diff = diff();

    let prompt = format!("{PROMPT}\n{diff}");

    let ai_msg = ai(prompt);

    match ai_msg {
        Some(msg) => {
            match msg.as_str() {
                "" => {
                    println!("EMPTY AI MESSAGE, DEFAULT WIP");
                    "WIP".to_string()
                },
                str=> {
                    println!("AI MESSAGE");
                    str.to_string()
                },
            }
        }
        None => {
            println!("AI MESSAGE DIDN'T WORK, DEFAULT WIP");
            "WIP".to_string()
        }
    }
}

fn ai(prompt: String) -> Option<String>
{
    let mut ollama = match Command::new("ollama")
        .args(["run", "mistral"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => return None,
    };

    if ollama.stdin.as_mut()
        .and_then(|stdin| stdin.write_all(prompt.as_bytes()).ok())
        .is_none()
    {
        return None;
    }

    let a = String::from_utf8(
        ollama.wait_with_output().unwrap().stdout
    );


    if a.is_err()
    {
        return None;
    }


    Some(a.unwrap().trim().to_string())
}

fn diff() -> String
{
    String::from_utf8(
        Command::new("git").args(["diff"]).output().unwrap().stdout
    ).unwrap()
}
