use std::fmt;
use crate::config::Config;

pub fn tcr_cmd(config: fn() -> Option<Config>) -> Result<TcrCommand, ConfigurationNotFound>
{
    let result = config();
    if result.is_none()
    {
        return Err(ConfigurationNotFound);
    }
    let config = result.unwrap();
    let test = config.test;
    let commit = ["git add . && git commit", "-m WIP"]
        .into_iter()
        .chain(config.no_verify.then_some("--no-verify"))
        .collect::<Vec<_>>()
        .join(" ");
    let revert = "(git clean -fdq . && git reset --hard)";

    let cmd = format!(
        "({} && {} || {})", test, commit, revert);
    Ok(
        vec![config.before, vec![cmd]]
            .concat()
            .join(" && "))
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
                before: vec![
                    String::from("pnpm tc"),
                    String::from("prettier --write .")
                ],
                no_verify: false
            })
        }

        let result = tcr::tcr_cmd(test_conf);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "pnpm tc && prettier --write . && (pnpm test && git add . && git commit -m WIP || (git clean -fdq . && git reset --hard))");
    }

    #[test]
    fn it_runs_tcr_with_empty_before()
    {
        fn test_conf() -> Option<Config>
        {
            Some(Config
            {
                test: String::from("pnpm test"),
                before: vec![],
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
                before: vec![],
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
