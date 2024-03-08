extern crate core;

use std::io;
use std::io::Write;
use std::process::Command;

fn main()
{
    let output = Command::new("sh")
        .arg("-c")
        .arg(tcr())
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap()
}

fn tcr() -> &'static str
{
    return "cal"
}

#[cfg(test)]
mod tests
{
    use crate::tcr;

    #[test]
    fn it_works()
    {
        assert_eq!(tcr(), "cal");
    }
}
