use std::collections::HashMap;

#[macro_use]
extern crate serde;

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(rename = "pkg")]
    packages: HashMap<String, HashMap<String, PackageAuthorConfig>>,
}

#[derive(Debug, Deserialize)]
struct PackageAuthorConfig {
    repo: Option<String>,
}

#[derive(Debug, Deserialize)]
enum PackageConfig {
    Basic(String),
    // Advanced {
    //     repo: String,
    // }
}

fn read_config() -> Config {
    let cfg_bytes = std::fs::read("config.toml").unwrap();
    toml::de::from_slice(&cfg_bytes).unwrap()
}

fn main() {
    let config = read_config();

    dbg!(config);
}
