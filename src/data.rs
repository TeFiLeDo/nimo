use std::collections::BTreeMap;

use chrono::{DateTime, Utc};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Data {
    #[serde(default)]
    pub pings: BTreeMap<DateTime<Utc>, crate::ping::Data>,
    #[serde(default)]
    pub speed_tests: BTreeMap<DateTime<Utc>, crate::speedtest::Data>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            pings: BTreeMap::new(),
            speed_tests: BTreeMap::new(),
        }
    }
}
