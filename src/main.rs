extern crate core;

use std::env::current_dir;
use std::process::{Command, ExitCode};
use crate::config::yaml_config;
use crate::message::wip;
use crate::tcr::tcr;
use clap::Parser;

mod tcr;
mod config;
mod git;
mod message;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, value_name = "TRAILER", num_args = 0.., action = clap::ArgAction::Append)]
    trailer: Vec<String>,
    #[arg(short = 'm', long = "message", value_name = "MESSAGE", help = "Custom commit message (overrides WIP)")]
    message: Option<String>,
}

fn main() -> ExitCode
{
    let cli = Cli::parse();
    match yaml_config(current_dir().unwrap()) {
        Some(configuration) => {
            let message_fn: Box<dyn Fn(&str) -> String> = match cli.message {
                Some(msg) => Box::new(move |_diff: &str| msg.clone()),
                None => Box::new(wip),
            };
            let git = git::GitRepository {
                config: configuration.clone(),
                exec: Box::new(|cmd: &mut Command| {
                    cmd.output()
                }),
                message: message_fn,
                trailers: cli.trailer.clone(),
            };
            if tcr(&git) {
                ExitCode::SUCCESS
            }
            else {
                ExitCode::FAILURE
            }
        },
        None => {
            eprintln!("\
            Error: 'tcr.yaml' not found in the current directory. \
            Please create the configuration file to proceed.");
            ExitCode::FAILURE
        }
    }
}
