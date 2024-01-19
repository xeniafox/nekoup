use std::fs;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NekoupConfig {
    #[allow(dead_code)]
    pub backend: NekoupBackendConfig,
}

#[derive(Debug, Deserialize)]
pub struct NekoupBackendConfig {
    #[allow(dead_code)]
    pub host: String,

    #[allow(dead_code)]
    pub upload_dir: String,
}

pub fn load_config() -> NekoupConfig {
    let config_file = concat!("data", "/", "nekoup.toml"); // Hardcoded config file path

    let toml_contents =
        fs::read_to_string(config_file).expect("Unable to open nekoup.toml config file!");

    let config: NekoupConfig =
        toml::from_str(&toml_contents).expect("Failed to parse config file!");

    return config;
}
