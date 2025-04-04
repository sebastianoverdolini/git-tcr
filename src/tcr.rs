use std::fmt;
use crate::config::Config;

pub fn tcr_cmd(config: fn() -> Option<Config>) -> Result<TcrCommand, ConfigurationNotFound>
{
    let config = config().ok_or(ConfigurationNotFound)?;

    let test = test_command(config.test);
    let commit = commit_command(config.no_verify.unwrap_or(false));
    let revert = revert_command();

    Ok(format!("({test} && {commit} || {revert})"))
}

fn test_command(test: String) -> String {
    test
}

fn commit_command(no_verify: bool) -> String {
    std::iter::once("git add . && git commit -m WIP")
        .chain(no_verify.then_some("--no-verify"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn revert_command() -> &'static str {
    "(git clean -fdq . && git reset --hard)"
}

pub type TcrCommand = String;

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
            "(pnpm test && git add . && git commit -m WIP || (git clean -fdq . && git reset --hard))");
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
            "(npm test && git add . && git commit -m WIP --no-verify || (git clean -fdq . && git reset --hard))");
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
            "(npm test && git add . && git commit -m WIP || (git clean -fdq . && git reset --hard))");
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
