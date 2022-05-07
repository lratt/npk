use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "pkg")]
    pub packages: HashMap<String, HashMap<String, Package>>,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub rename: Option<String>,
}

pub fn read<P: AsRef<Path>>(path: P) -> Result<Config> {
    let cfg_bytes = std::fs::read(path)?;
    Ok(toml::de::from_slice(&cfg_bytes)?)
}
