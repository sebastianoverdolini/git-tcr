use std::fmt;
use crate::config::Config;

pub fn tcr_cmd(config: fn() -> Option<Config>) -> Result<TcrCommand, ConfigurationNotFound>
{
    let config = config().ok_or(ConfigurationNotFound)?;

    Ok(tcr_command(
        || test_command(config.clone().test),
        || commit_command(config.clone().no_verify),
        || revert_command()))
}

fn tcr_command(
    test_command: impl Fn() -> TestCommand,
    commit_command: impl Fn() -> CommitCommand,
    revert_command: impl Fn() -> RevertCommand
) -> TcrCommand
{
    // TODO To test independently
    let test = test_command();
    let commit = commit_command();
    let revert = revert_command();

    format!("git add . &&  [ -n \"$(git status --porcelain)\" ] && ({test} && {commit} || {revert})")
}

fn test_command(test: String) -> TestCommand {
    test
}

fn commit_command(no_verify: Option<bool>) -> CommitCommand {
    std::iter::once("git commit -m WIP")
        .chain(no_verify.unwrap_or(false).then_some("--no-verify"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn revert_command() -> RevertCommand {
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
mod tcr_tests
{
    use crate::tcr;
    use crate::config::Config;
    use crate::tcr::ConfigurationNotFound;

    #[test]
    fn it_returns_tcr_command()
    {
        let result = tcr::tcr_cmd(|| Some(Config
        {
            test: String::from("pnpm test"),
            no_verify: None
        }));

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "git add . &&  [ -n \"$(git status --porcelain)\" ] && (pnpm test && git commit -m WIP || (git clean -fdq . && git reset --hard))");
    }

    #[test]
    fn it_returns_tcr_command_with_no_verify_when_option_is_true()
    {
        let result = tcr::tcr_cmd(|| Some(Config
        {
            test: String::from("npm test"),
            no_verify: Some(true)
        }));

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "git add . &&  [ -n \"$(git status --porcelain)\" ] && (npm test && git commit -m WIP --no-verify || (git clean -fdq . && git reset --hard))");
    }

    #[test]
    fn it_returns_tcr_command_with_verify_when_option_is_false()
    {
        let result = tcr::tcr_cmd(|| Some(Config
        {
            test: String::from("npm test"),
            no_verify: Some(false)
        }));

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "git add . &&  [ -n \"$(git status --porcelain)\" ] && (npm test && git commit -m WIP || (git clean -fdq . && git reset --hard))");
    }


    #[test]
    fn it_returns_error_if_configuration_is_not_present()
    {
        let result = tcr::tcr_cmd(|| None);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ConfigurationNotFound)
    }
}

#[cfg(test)]
mod tcr_command_test
{
    use crate::tcr::{tcr_command};

    #[test]
    fn cmd()
    {
        let cmd = tcr_command(
            || "test".to_string(),
            || "commit".to_string(),
            || "revert".to_string());

        assert_eq!(
            cmd,
            "git add . &&  [ -n \"$(git status --porcelain)\" ] && (test && commit || revert)");
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
