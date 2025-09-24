extern crate core;

use std::env::current_dir;
use std::process::{Command};
use crate::commit::commit_command;
use crate::config::{yaml_config};
use crate::revert::revert_command;
use crate::tcr::{tcr};
use crate::test::test_command;

mod tcr;
mod config;
mod test;
mod revert;
mod commit;

fn main()
{
    _tcr()
}

fn _tcr() {
    let config = || yaml_config(current_dir().unwrap());
    match tcr(
        config,
        test_command,
        commit_command,
        revert_command,
        |program, args| {
            Command::new(program)
                .args(args)
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .output()
        }) {
        Ok(()) =>
            println!("Done"),
        Err(configuration_not_found) =>
            eprintln!("{}", configuration_not_found)
    }
}
