pub type Commit = fn(CommitConfig) -> CommitCommand;

#[derive(Debug)]
pub struct CommitConfig
{
    pub no_verify: Option<bool>
}

pub type CommitCommand = String;

pub fn commit_command(config: CommitConfig) -> CommitCommand {
    std::iter::once("git commit -m WIP")
        .chain(config.no_verify.unwrap_or(false).then_some("--no-verify"))
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
            CommitConfig { no_verify: Some(false) });

        assert_eq!(cmd, "git commit -m WIP");
    }

    #[test]
    fn none()
    {
        let cmd = commit_command(
            CommitConfig { no_verify: None });

        assert_eq!(cmd, "git commit -m WIP");
    }

    #[test]
    fn no_verify()
    {
        let cmd = commit_command(
            CommitConfig { no_verify: Some(true) });

        assert_eq!(cmd, "git commit -m WIP --no-verify");
    }
}
