use std::fmt;
use crate::config::Config;

pub fn tcr_cmd(config: fn() -> Option<Config>) -> Result<TcrCommand, ConfigurationNotFound>
{
    let config = config().ok_or(ConfigurationNotFound)?;

    let test = test_command(config.test);
    let commit = commit_command(config.no_verify);
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
    fn it_runs_tcr()
    {
        fn test_conf() -> Option<Config>
        {
            Some(Config
            {
                test: String::from("pnpm test"),
                no_verify: false
            })
        }

        let result = tcr::tcr_cmd(test_conf);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "(pnpm test && git add . && git commit -m WIP || (git clean -fdq . && git reset --hard))");
    }

    #[test]
    fn it_runs_tcr_with_no_verify()
    {
        fn test_conf() -> Option<Config>
        {
            Some(Config
            {
                test: String::from("npm test"),
                no_verify: true
            })
        }

        let result = tcr::tcr_cmd(test_conf);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "(npm test && git add . && git commit -m WIP --no-verify || (git clean -fdq . && git reset --hard))");
    }


    #[test]
    fn it_returns_error_if_configuration_is_not_present()
    {
        fn no_conf() -> Option<Config>
        {
            None
        }

        let result = tcr::tcr_cmd(no_conf);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ConfigurationNotFound)
    }
}
