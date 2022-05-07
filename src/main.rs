#![warn(clippy::pedantic)]

use anyhow::Context;
use installer::Installer;
use std::path::PathBuf;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate clap;

mod config;
mod installer;

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

fn main() -> anyhow::Result<()> {
    let matches = command!()
        .about("small cli package manager for neovim")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .args(&[arg!(
            -c --config <CONFIG_FILE> "path to configuration file [default: $HOME/.config/pkg-nvim.toml]"
        ).allow_invalid_utf8(true).required(false)])
        .subcommands(vec![
            clap::Command::new("install").about("installs all new packages").visible_alias("i")
                .args(&[arg!(-u --upgrade "upgrade existing packages").required(false)]),
            clap::Command::new("upgrade").about("updates all existing packages").visible_alias("u"),
        ])
        .get_matches();

    let config_path = matches.value_of_os("config").map_or_else(
        || home::home_dir().unwrap().join(".config/pkg-nvim.toml"),
        PathBuf::from,
    );

    let config = config::read(&config_path).context("failed to read config file")?;

    let mut installer = Installer::new(config);
    if let Some(install) = matches.subcommand_matches("install") {
        let upgrade = install.is_present("upgrade");
        installer.set_upgrade_during_install(upgrade);
        installer.all_repos(Installer::clone_repo)?;
    }
    if let Some(_upgrade) = matches.subcommand_matches("upgrade") {
        installer.all_repos(Installer::pull_repo)?;
    }

    Ok(())
}
