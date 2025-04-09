use serde::{Deserialize, Serialize};

pub type Commit = fn(CommitConfig) -> CommitCommand;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CommitConfig
{
    pub no_verify: Option<bool>
}

pub type CommitCommand = String;

pub fn commit_command(msg: fn() -> String, config: CommitConfig) -> CommitCommand {
    let msg = msg();

    std::iter::once(format!("git commit -m \"{msg}\""))
        .chain(config.no_verify.unwrap_or(false).then_some("--no-verify".to_string()))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod commit_command_test
{
    use crate::commit::{commit_command, CommitConfig};

    #[test]
    fn verifying()
    {
        let cmd = commit_command(
            || "WIP".to_string(),
            CommitConfig { no_verify: Some(false) });

        assert_eq!(cmd, "git commit -m \"WIP\"");
    }

    #[test]
    fn none()
    {
        let cmd = commit_command(
            || "WIP".to_string(),
            CommitConfig { no_verify: None });

        assert_eq!(cmd, "git commit -m \"WIP\"");
    }

    #[test]
    fn no_verify()
    {
        let cmd = commit_command(
            || "WIP".to_string(),
            CommitConfig { no_verify: Some(true) });

        assert_eq!(cmd, "git commit -m \"WIP\" --no-verify");
    }
}
