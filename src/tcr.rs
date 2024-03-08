use crate::config::Config;

pub fn tcr(config: fn() -> Option<Config>) -> Result<String, String>
{
    let result = config();
    if result.is_none()
    {
        return Err(String::from(""))
    }
    let config = result.unwrap();
    return Ok(format!(
        "{}({} && git add . && git commit -m WIP || git reset --hard)",
        if !config.before.is_empty() { config.before.join(" && ") + " && " } else { "".to_string() },
        config.test));
}

#[cfg(test)]
mod tcr_tests
{
    use crate::tcr;
    use crate::config::Config;


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

        let result = tcr::tcr(test_conf);

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

        let result = tcr::tcr(test_conf);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "(pnpm test && git add . && git commit -m WIP || git reset --hard)");
    }

    #[test]
    fn it_returns_error_if_configuration_is_not_present()
    {
        fn no_conf() -> Option<Config>
        {
            return None
        }

        assert!(tcr::tcr(no_conf).is_err());
    }
}
