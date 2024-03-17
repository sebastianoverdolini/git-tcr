extern crate core;

use std::process::Command;
use crate::config::yaml_config;

mod tcr;
mod config;

fn main()
{
    let result = tcr::tcr(|| yaml_config(String::from(".")));
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
        Err(_) => println!("{}", "Configuration not found.")
    }

}
