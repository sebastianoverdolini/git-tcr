extern crate core;

use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main()
{
    let tcr = tcr(|| config(String::from("."))).expect("");
    let output = Command::new("sh")
        .arg("-c")
        .arg(tcr)
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap()
}

#[derive(Debug, PartialEq)]
struct Config
{
    test: String
}

fn config(location: String) -> Option<Config>
{
    let config_path = location.to_owned() + "/tcr.config";
    if !Path::new(&(config_path)).exists()
    {
        return None;
    }
    return Some(Config
    {
        test: String::from(
            std::fs::read_to_string(config_path)
                .unwrap()
                .lines()
                .next()
                .expect(""))
    });
}

fn tcr(config: fn() -> Option<Config>) -> Result<String, String>
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
    use crate::{Config, tcr};


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

        let result = tcr(test_conf);

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

        assert!(tcr(no_conf).is_err());
    }
}

#[cfg(test)]
mod file_config_tests
{
    use std::fs::{create_dir_all, remove_dir_all, write};
    use crate::{config, Config};

    #[test]
    fn it_returns_the_content_of_the_config_if_the_file_is_present_in_the_current_location()
    {
        create_dir_all("test-env").expect("TODO: panic message");
        write("test-env/tcr.config", "npm test\n").expect("TODO: panic message");

        let config = config(String::from("./test-env"));

        assert!(config.is_some());
        assert_eq!(
            config.unwrap(),
            Config
            {
                test: String::from("npm test")
            });

        remove_dir_all("test-env").expect("TODO: panic message");
    }

    #[test]
    fn it_returns_none_if_the_config_file_is_not_present_in_the_current_location()
    {
        assert!(config(String::from("./test-env")).is_none());
    }
}
