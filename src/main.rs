extern crate core;

use std::env::current_dir;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;
use clap::Parser;
use notify::RecursiveMode;
use notify_debouncer_full::new_debouncer;
use commit::commit_command;
use revert::revert_command;
use test::test_command;
use crate::config::yaml_config;
use crate::tcr::tcr_command;

mod tcr;
mod config;
mod test;
mod revert;
mod commit;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    watch: bool,
}

fn main()
{
    let args = Args::parse();

    if args.watch
    {
        watched(tcr)
    }
    else
    {
        tcr()
    }
}

fn tcr() {
    let result = tcr_command(
        || yaml_config(current_dir().unwrap()),
        test_command,
        commit_command,
        revert_command);
    match result {
        Ok(cmd) => {
            Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .spawn()
                .expect("failed to execute process")
                .wait()
                .expect("TODO: panic message");
        }
        Err(error) => println!("Error: {}", error)
    }
}

fn watched(tcr: fn()) {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut debouncer = new_debouncer(Duration::from_secs(1), None, tx).unwrap();
    debouncer
        .watch(Path::new("."), RecursiveMode::Recursive)
        .unwrap();

    for result in rx {
        match result {
            Ok(events) => {
                let is_considered = events.iter()
                    .any(|e| e.paths.iter().any(|path|
                        !is_within_git_directory(path) &&
                            !is_git_ignored(path)
                    ));
                if is_considered {
                    tcr();
                }
            },
            Err(error) => println!("Error {error:?}"),
        }
    }
}

fn is_within_git_directory(path: &Path) -> bool {
    let path_str = path.to_str().unwrap();
    path_str.contains("/.git/") || path_str == ".git"
}

fn is_git_ignored(path: &Path) -> bool {
    let path_str = path.to_str().unwrap();

    let output = Command::new("git")
        .arg("check-ignore")
        .arg(path_str)
        .stderr(Stdio::null())
        .output();

    match output {
        Ok(output) => !output.stdout.is_empty(),
        Err(_) => false,
    }
}
