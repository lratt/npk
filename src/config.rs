use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "pkg")]
    pub packages: HashMap<String, HashMap<String, PackageAuthorConfig>>,
}

#[derive(Debug, Deserialize)]
pub struct PackageAuthorConfig {
    pub repo: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum PackageConfig {
    Basic(String),
    // Advanced {
    //     repo: String,
    // }
}

pub fn read_config() -> Config {
    let cfg_bytes = std::fs::read("config.toml").unwrap();
    toml::de::from_slice(&cfg_bytes).unwrap()
}

