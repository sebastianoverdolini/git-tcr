extern crate core;

use std::io;
use std::io::Write;
use std::process::Command;

fn main()
{
    let output = Command::new("sh")
        .arg("-c")
        .arg(tcr(config).expect(""))
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap()
}

fn config() -> Option<String>
{
    Some(String::from("cargo test"))
}

#[allow(unused)]
fn tcr(config: fn() -> Option<String>) -> Result<String, String>
{
    if (config().is_none())
    {
        return Err(String::from(""))
    }
    return Ok(format!(
        "{} && git add . && git commit -m WIP || git reset --hard",
        config().expect("")));
}

#[cfg(test)]
mod tests
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
