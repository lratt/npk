use anyhow::Context;

#[macro_use]
extern crate serde;

mod config;

fn main() -> anyhow::Result<()> {
    let config = config::read_config().context("failed to read config file")?;

    dbg!(config);

    Ok(())
}
