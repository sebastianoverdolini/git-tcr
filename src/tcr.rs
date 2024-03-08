use crate::config::Config;

pub fn tcr(config: fn() -> Option<Config>) -> Result<String, String>
{
    let config = config();
    if config.is_none()
    {
        return Err(String::from(""))
    }
    return Ok(format!(
        "{} && git add . && git commit -m WIP || git reset --hard",
        config.unwrap().test));
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
                test: String::from("cargo test")
            })
        }

        let result = tcr::tcr(test_conf);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "cargo test && git add . && git commit -m WIP || git reset --hard");
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
