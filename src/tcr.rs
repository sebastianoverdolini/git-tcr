use crate::config::Config;

pub fn tcr(config: fn() -> Option<Config>, args: Vec<String>) -> Result<String, ConfigurationNotFound>
{
    let result = config();
    if result.is_none()
    {
        return Err(ConfigurationNotFound);
    }
    let config = result.unwrap();
    let plain_tcr = format!(
        "({} && git add . && git commit -m WIP{} || git reset --hard)",
        config.test,
        if args.contains(&"--push".to_string()) { " && git push" } else { "" });
    return Ok(
        vec![config.before, vec![plain_tcr]]
            .concat()
            .join(" && "));
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConfigurationNotFound;

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

        let result = tcr::tcr(test_conf, no_args());

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "pnpm tc && prettier --write . && (pnpm test && git add . && git commit -m WIP || git reset --hard)");
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

        let result = tcr::tcr(test_conf, no_args());

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "(pnpm test && git add . && git commit -m WIP || git reset --hard)");
    }

    #[test]
    fn it_runs_tcr_appending_push_if_arg_is_present()
    {
        fn test_conf() -> Option<Config>
        {
            return Some(Config
            {
                test: String::from("pnpm test"),
                before: vec![]
            })
        }

        let result = tcr::tcr(test_conf, arg("--push"));

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "(pnpm test && git add . && git commit -m WIP && git push || git reset --hard)");
    }

    #[test]
    fn it_returns_error_if_configuration_is_not_present()
    {
        fn no_conf() -> Option<Config>
        {
            return None
        }

        let result = tcr::tcr(no_conf, no_args());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ConfigurationNotFound)
    }

    fn no_args() -> Vec<String>
    {
        return vec!["target/debug/git-tcr", "tcr"]
            .iter()
            .map(|s| s.to_string())
            .collect();
    }

    fn arg(arg: &str) -> Vec<String>
    {
        return vec!["target/debug/git-tcr", "tcr", arg]
            .iter()
            .map(|s| s.to_string())
            .collect();
    }
}
