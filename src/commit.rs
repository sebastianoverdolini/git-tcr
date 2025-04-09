pub type Commit = fn(no_verify: Option<bool>) -> CommitCommand;

pub fn commit_command(no_verify: Option<bool>) -> CommitCommand {
    std::iter::once("git commit -m WIP")
        .chain(no_verify.unwrap_or(false).then_some("--no-verify"))
        .collect::<Vec<_>>()
        .join(" ")
}

pub type CommitCommand = String;

#[cfg(test)]
mod commit_command_test
{
    use crate::commit::commit_command;

    #[test]
    fn verifying()
    {
        let cmd = commit_command(Some(false));

        assert_eq!(cmd, "git commit -m WIP");
    }

    #[test]
    fn none()
    {
        let cmd = commit_command(None);

        assert_eq!(cmd, "git commit -m WIP");
    }

    #[test]
    fn no_verify()
    {
        let cmd = commit_command(Some(true));

        assert_eq!(cmd, "git commit -m WIP --no-verify");
    }
}
