use std::{
    collections::BTreeMap,
    net::{IpAddr, Ipv4Addr},
};

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(default = "default_count")]
    pub count: u8,
    #[serde(default = "default_targets")]
    pub targets: BTreeMap<String, IpAddr>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            count: default_count(),
            targets: default_targets(),
        }
    }
}

fn default_count() -> u8 {
    16
}

fn default_targets() -> BTreeMap<String, IpAddr> {
    let mut ret = BTreeMap::new();

    ret.insert("cloudflare".to_string(), Ipv4Addr::new(1, 1, 1, 1).into());
    ret.insert("google".to_string(), Ipv4Addr::new(8, 8, 8, 8).into());

    ret
}
