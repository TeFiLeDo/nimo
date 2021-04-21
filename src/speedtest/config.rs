#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: default_enabled(),
        }
    }
}

fn default_enabled() -> bool {
    false
}
