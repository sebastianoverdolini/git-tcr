extern crate core;

use std::io;
use std::io::Write;
use std::process::Command;

fn main()
{
    let output = Command::new("sh")
        .arg("-c")
        .arg(tcr(config))
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap()
}

fn config() -> String
{
    String::from("cargo test")
}

#[allow(unused)]
fn tcr(config: fn() -> String) -> String
{
    return format!("{} && git add . && git commit -m WIP || git reset --hard", config());
}

#[cfg(test)]
mod tests
{
    use crate::tcr;


    #[test]
    fn it_runs_tcr()
    {
        fn test_conf() -> String
        {
            return String::from("cargo test")
        }
        assert_eq!(tcr(test_conf), "cargo test && git add . && git commit -m WIP || git reset --hard");
    }
}
