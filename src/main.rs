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

fn config(location: String) -> Option<String>
{
    let config_path = location.to_owned() + "/tcr.config";
    if !Path::new(&(config_path)).exists()
    {
        return None;
    }
    return Some(String::from(std::fs::read_to_string(config_path).unwrap().lines().next().expect("")));
}

fn tcr(config: fn() -> Option<String>) -> Result<String, String>
{
    if config().is_none()
    {
        return Err(String::from(""))
    }
    return Ok(format!(
        "{} && git add . && git commit -m WIP || git reset --hard",
        config().expect("")));
}

#[cfg(test)]
mod tcr_tests
{
    use crate::tcr;


    #[test]
    fn it_runs_tcr()
    {
        fn test_conf() -> Option<String>
        {
            return Some(String::from("cargo test"))
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
        fn no_conf() -> Option<String>
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
    use crate::config;

    #[test]
    fn it_returns_the_content_of_the_config_if_the_file_is_present_in_the_current_location()
    {
        create_dir_all("test-env").expect("TODO: panic message");
        write("test-env/tcr.config", "npm test\n").expect("TODO: panic message");

        let config = config(String::from("./test-env"));
        assert!(config.is_some());
        assert_eq!(config.unwrap(), "npm test");

        remove_dir_all("test-env").expect("TODO: panic message");
    }

    #[test]
    fn it_returns_none_if_the_config_file_is_not_present_in_the_current_location()
    {
        assert!(config(String::from("./test-env")).is_none());
    }
}
