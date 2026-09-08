extern crate core;

use std::env::current_dir;
use std::io;
use std::process::{Command, ExitCode};
use crate::config::yaml_config;
use crate::message::wip;
use crate::tcr::tcr;
use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};

mod tcr;
mod config;
mod git;
mod init;
mod message;

#[derive(Parser)]
#[command(author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(long, value_name = "TRAILER", num_args = 0.., action = clap::ArgAction::Append)]
    trailer: Vec<String>,
    #[arg(short = 'm', long = "message", value_name = "MESSAGE", help = "Custom commit message (overrides WIP)")]
    message: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Interactively create a tcr.yaml configuration file
    Init,
}

fn main() -> ExitCode
{
    // -V/--version stays the bare crate version (some tooling parses it);
    // --version's long form additionally names the supported tcr.yaml
    // schema version, so `git tcr --version` alone tells you both.
    let long_version = format!(
        "{} (tcr.yaml schema version {})",
        env!("CARGO_PKG_VERSION"),
        config::MAX_SUPPORTED_VERSION,
    );
    let matches = Cli::command()
        .version(env!("CARGO_PKG_VERSION"))
        .long_version(long_version.leak() as &str)
        .get_matches();
    let cli = Cli::from_arg_matches(&matches).unwrap_or_else(|err| err.exit());
    match cli.command {
        Some(Commands::Init) => {
            let stdin = io::stdin();
            let mut input = stdin.lock();
            let mut output = io::stdout();
            match init::init(current_dir().unwrap(), &mut input, &mut output, &|cmd: &mut Command| cmd.status()) {
                Ok(true) => ExitCode::SUCCESS,
                Ok(false) => ExitCode::FAILURE,
                Err(err) => {
                    eprintln!("Error: {err}");
                    ExitCode::FAILURE
                }
            }
        }
        None => match yaml_config(current_dir().unwrap()) {
            Ok(configuration) => {
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
            Err(err) => {
                eprintln!("Error: {err}");
                ExitCode::FAILURE
            }
        }
    }
}
