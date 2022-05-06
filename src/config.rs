use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "pkg")]
    pub packages: HashMap<String, HashMap<String, Package>>,
}

#[derive(Debug, Deserialize)]
pub struct Package {}

pub fn read() -> Result<Config> {
    let cfg_bytes = std::fs::read("config.toml")?;
    Ok(toml::de::from_slice(&cfg_bytes)?)
}
