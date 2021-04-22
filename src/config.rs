use std::path::PathBuf;

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
