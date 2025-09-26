extern crate core;

use std::env::{self, current_dir};
use std::process::Command;
use crate::config::yaml_config;
use crate::message::{scribe, wip};
use crate::tcr::tcr;

mod tcr;
mod config;
mod git;
mod message;

fn main()
{
    let use_scribe = env::args().any(|arg| arg == "--scribe");
    match yaml_config(current_dir().unwrap()) {
        Some(configuration) => {
            let git = git::GitRepository {
                config: configuration.clone(),
                exec: Box::new(|cmd: &mut Command| {
                    cmd
                        .output()
                }),
                message: if use_scribe { scribe } else { wip },
            };
            tcr(&git)
        },
        None => eprintln!("Configuration not found.")
    }
}
