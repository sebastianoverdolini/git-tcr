extern crate core;

use std::env::current_dir;
use std::process::Command;
use crate::config::yaml_config;
use crate::tcr::{tcr_cmd, TcrCommand};

mod tcr;
mod config;

fn main()
{
    let result = tcr_cmd(|| yaml_config(current_dir().unwrap()));
    match result {
        Ok(cmd) => sh(cmd),
        Err(error) => println!("Error: {}", error)
    }
}

fn sh(cmd: TcrCommand)
{
    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("TODO: panic message");
}
