use std::fmt;
use crate::config::Config;

pub fn tcr_command(
    config: fn() -> Option<Config>,
    test_command: fn(test: String) -> TestCommand,
    commit_command: fn(no_verify: Option<bool>) -> CommitCommand,
    revert_command: fn() -> RevertCommand
) -> Result<TcrCommand, ConfigurationNotFound>
{
    let config = config().ok_or(ConfigurationNotFound)?;

    let test = test_command(config.clone().test);
    let commit = commit_command(config.clone().no_verify);
    let revert = revert_command();

    Ok(format!("git add . &&  [ -n \"$(git status --porcelain)\" ] && ({test} && {commit} || {revert})"))
}

pub fn test_command(test: String) -> TestCommand {
    test
}

pub fn commit_command(no_verify: Option<bool>) -> CommitCommand {
    std::iter::once("git commit -m WIP")
        .chain(no_verify.unwrap_or(false).then_some("--no-verify"))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn revert_command() -> RevertCommand {
    "(git clean -fdq . && git reset --hard)".to_string()
}

pub type TcrCommand = String;

pub type TestCommand = String;

pub type CommitCommand = String;

pub type RevertCommand = String;

#[derive(Debug, Clone, PartialEq)]
pub struct ConfigurationNotFound;

impl fmt::Display for ConfigurationNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Configuration not found.")
    }
}

#[cfg(test)]
mod tcr_command_test
{
    use crate::config::Config;
    use crate::tcr::{tcr_command, ConfigurationNotFound};

    #[test]
    fn cmd()
    {
        // TODO Test better with mocks?
        let cmd = tcr_command(
            || Some(Config
            {
                test: String::from("test"),
                no_verify: Some(true)
            }),
            |test| format!("{test}").to_string(),
            |no_verify| format!("commit {no_verify:?}").to_string(),
            || "revert".to_string());

        assert!(cmd.is_ok());
        assert_eq!(
            cmd.unwrap(),
            "git add . &&  [ -n \"$(git status --porcelain)\" ] && (test && commit Some(true) || revert)");
    }

    #[test]
    fn no_conf()
    {
        // TODO Throw exception on usages?
        let cmd = tcr_command(
            || None,
            |_test| "".to_string(),
            |_no_verify| "".to_string(),
            || "".to_string());

        assert!(cmd.is_err());
        assert_eq!(cmd.unwrap_err(), ConfigurationNotFound);
    }
}

#[cfg(test)]
mod test_command_test
{
    use crate::tcr::test_command;

    #[test]
    fn cmd()
    {
        let cmd = test_command("pnpm test".to_string());

        assert_eq!(cmd, "pnpm test");
    }
}

#[cfg(test)]
mod commit_command_test
{
    use crate::tcr::commit_command;

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

#[cfg(test)]
mod revert_command_test
{
    use crate::tcr::revert_command;

    #[test]
    fn cmd()
    {
        let cmd = revert_command();

        assert_eq!(cmd, "(git clean -fdq . && git reset --hard)");
    }
}
