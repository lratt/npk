#![warn(clippy::pedantic)]

use anyhow::Context;
use installer::Installer;

#[macro_use]
extern crate serde;

mod config;
mod installer;

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

fn print_usage(cmd: &str) {
    println!(
        "USAGE:
    {} [command]

COMMANDS:
    install, i    installs all new packages
    upgrade, u     updates current packages
    help, h       prints this message
",
        cmd
    );
}

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        print_usage(&args[0]);
        return Ok(());
    }
    let cmd = &args[1];

    let config = config::read().context("failed to read config file")?;

    let installer = Installer::new(config);
    match &cmd[..] {
        "i" | "install" => installer.all_repos(Installer::clone_repo)?,
        "u" | "upgrade" => installer.all_repos(Installer::pull_repo)?,
        "h" | "help" => print_usage(&args[0]),
        _ => {
            println!("invalid command");
            print_usage(&args[0]);
        }
    }

    Ok(())
}
