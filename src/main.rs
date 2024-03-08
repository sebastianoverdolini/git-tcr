extern crate core;

use std::io;
use std::io::Write;
use std::process::Command;

mod tcr;
mod config;

fn main()
{
    let tcr = tcr::tcr(|| config::yaml_config(String::from("."))).expect("");
    let output = Command::new("sh")
        .arg("-c")
        .arg(tcr)
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap()
}
