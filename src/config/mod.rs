pub mod settings;
use anyhow::{Context, Result};
use dirs::config_dir;
use settings::Config;
use std::fs;
use std::path::PathBuf;

pub fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;

    if config_path.exists() {
        let contents = fs::read_to_string(config_path)?;
        Ok(toml::from_str(&contents)?)
    } else {
        Ok(Config::default())
    }
}

fn get_config_path() -> Result<PathBuf> {
    let mut path = config_dir().context("Could not find configuration directory.")?;
    path.push("carbonrs");
    path.push("config.toml");
    Ok(path)
}
