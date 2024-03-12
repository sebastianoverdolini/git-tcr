extern crate core;

use std::env::args;
use std::process::Command;

mod tcr;
mod config;

fn main()
{
    let result = tcr::tcr(
        || config::yaml_config(String::from(".")),
        args().collect());
    match result {
        Ok(cmd) => {
            let mut child = Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .spawn()
                .expect("failed to execute process");
            child.wait().expect("TODO: panic message");
            child.kill().expect("TODO: panic message");
        }
        Err(_) => println!("{}", "Configuration not found.")
    }

}
