extern crate core;

use std::io;
use std::io::Write;
use std::process::Command;

mod tcr;
mod config;

fn main()
{
    let result = tcr::tcr(|| config::yaml_config(String::from(".")));
    match result {
        Ok(cmd) => {
            let output = Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .output()
                .expect("failed to execute process");
            io::stdout().write_all(&output.stdout).unwrap();
            io::stdout().write_all(&output.stderr).unwrap();
        }
        Err(_) => println!("{}", "Configuration not found.")
    }

}
