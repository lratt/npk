#![warn(clippy::pedantic)]

use anyhow::Context;

#[macro_use]
extern crate serde;

mod config;
mod installer;

fn main() -> anyhow::Result<()> {
    let config = config::read().context("failed to read config file")?;

    installer::Installer::new(config).clone_repos()?;

    Ok(())
}
