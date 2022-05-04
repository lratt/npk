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

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("an io error occured: {0}")]
    IoError(std::io::Error),
    #[error("toml deserialize error: {0}")]
    TomlDeserializeError(toml::de::Error),
}

#[derive(Debug, Deserialize)]
pub enum PackageConfig {
    Basic(String),
    // Advanced {
    //     repo: String,
    // }
}

pub fn read_config() -> Result<Config, ConfigError> {
    let cfg_bytes = std::fs::read("config.toml").map_err(ConfigError::IoError)?;
    toml::de::from_slice(&cfg_bytes).map_err(ConfigError::TomlDeserializeError)
}
