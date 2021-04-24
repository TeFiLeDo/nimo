#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default = "default_retries")]
    pub retries: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: default_enabled(),
            retries: default_retries(),
        }
    }
}

fn default_retries() -> u8 {
    4
}

fn default_enabled() -> bool {
    false
}
