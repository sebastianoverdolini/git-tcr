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
    let plain_tcr = format!(
        "git add -A && ({} && git commit -m WIP || git reset --hard)",
        config.test);
    return Ok(
        vec![config.before, vec![plain_tcr]]
            .concat()
            .join(" && "));
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
            return Some(Config
            {
                test: String::from("pnpm test"),
                before: vec![
                    String::from("pnpm tc"),
                    String::from("prettier --write .")
                ]
            })
        }

        let result = tcr::tcr_cmd(test_conf);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "pnpm tc && prettier --write . && git add -A && (pnpm test && git commit -m WIP || git reset --hard)");
    }

    #[test]
    fn it_runs_tcr_with_empty_before()
    {
        fn test_conf() -> Option<Config>
        {
            return Some(Config
            {
                test: String::from("pnpm test"),
                before: vec![]
            })
        }

        let result = tcr::tcr_cmd(test_conf);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "git add -A && (pnpm test && git commit -m WIP || git reset --hard)");
    }

    #[test]
    fn it_returns_error_if_configuration_is_not_present()
    {
        fn no_conf() -> Option<Config>
        {
            return None
        }

        let result = tcr::tcr_cmd(no_conf);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ConfigurationNotFound)
    }
}
