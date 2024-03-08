extern crate core;

use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use serde::{Serialize, Deserialize};

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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config
{
    pub test: String
}

fn config(location: String) -> Option<Config>
{
    let config_path = location.to_owned() + "/tcr.yaml";
    if !Path::new(&(config_path)).exists()
    {
        return None;
    }
    let content = std::fs::read_to_string(config_path).unwrap();
    let config = serde_yaml::from_str(&content).expect("");
    return Some(config)
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
        let c = Config
        {
            test: String::from("npm test")
        };
        let yaml = serde_yaml::to_string(&c).unwrap();
        write("test-env/tcr.yaml", yaml).expect("TODO: panic message");

        let result = config(String::from("./test-env"));

        assert!(result.is_some());
        assert_eq!(result.unwrap(), c);

        remove_dir_all("test-env").expect("TODO: panic message");
    }

    #[test]
    fn it_returns_none_if_the_config_file_is_not_present_in_the_current_location()
    {
        assert!(config(String::from("./test-env")).is_none());
    }
}
