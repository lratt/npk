#![warn(clippy::pedantic)]

use anyhow::Context;

#[macro_use]
extern crate serde;

mod config;
mod installer;

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

fn print_usage() {
    println!(
        "USAGE:
    {} [command]

COMMANDS:
    install, i    installs all new packages
    update, u     updates current packages
    help, h       prints this message
",
        PKG_NAME
    );
}

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }
    let cmd = &args[1];

    let config = config::read().context("failed to read config file")?;

    let installer = installer::Installer::new(config);
    match &cmd[..] {
        "i" | "install" => installer.clone_repos()?,
        "u" | "update" => unimplemented!(),
        "h" | "help" => print_usage(),
        _ => {
            println!("invalid command");
            print_usage();
        }
    }

    Ok(())
}
