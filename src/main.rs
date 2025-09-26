extern crate core;

use std::env::current_dir;
use std::process::Command;
use crate::config::yaml_config;
use crate::tcr::tcr;

mod tcr;
mod config;
mod git;

fn main()
{
    let config = yaml_config(current_dir().unwrap());
    match config {
        Some(configuration) => {
            let git = git::GitRepository {
                config: configuration.clone(),
                exec: Box::new(|program: &str, args: &[&str]| {
                    Command::new(program)
                        .args(args)
                        .stdout(std::process::Stdio::inherit())
                        .stderr(std::process::Stdio::inherit())
                        .output()
                }),
            };
            tcr(&git)
        },
        None => eprintln!("Configuration not found.")
    }
}
