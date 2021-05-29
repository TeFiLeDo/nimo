use std::path::PathBuf;

use anyhow::{Context, Result};
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(default = "default_data_path")]
    pub data: PathBuf,
    #[serde(default)]
    pub ping: crate::ping::Config,
    #[serde(default)]
    pub speed_test: crate::speedtest::Config,
}

fn default_data_path() -> PathBuf {
    PathBuf::from("/var/lib/nimo/data")
}

impl Config {
    pub fn load() -> Result<Self> {
        let mut cfg = Figment::new().merge(Toml::file("/etc/nimo.toml"));
        if let Some(mut x) = home::home_dir() {
            x.push(".config/nimo.toml");
            cfg = cfg.merge(Toml::file(x));
        }
        cfg = cfg.merge(Env::prefixed("NIMO_").split("_"));
        cfg.extract().context("failed to load configuration")
    }
}
