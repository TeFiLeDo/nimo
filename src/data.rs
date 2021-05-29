use std::{
    collections::BTreeMap,
    fs::{create_dir_all, File, OpenOptions},
    io::{BufReader, BufWriter, Seek, SeekFrom},
};

use crate::config::Config;
use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use fs2::FileExt;
use log::warn;
use ron::{
    de::from_reader,
    ser::{to_writer, to_writer_pretty, PrettyConfig},
};

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

    pub fn load_from_file(config: &Config, allow_write: bool) -> Result<(Self, File)> {
        if let Some(x) = config.data.parent() {
            if x.is_file() {
                return Err(anyhow!("data directory is file"));
            } else if !x.exists() {
                create_dir_all(x).context("failed to create data directory")?;
            }
        }

        let file = OpenOptions::new()
            .read(true)
            .write(allow_write)
            .create(allow_write)
            .open(&config.data)
            .context("failed to open data file")?;

        if allow_write {
            file.lock_exclusive()
        } else {
            file.lock_shared()
        }
        .context("failed to lock data file")?;

        let data = if config
            .data
            .metadata()
            .context("failed to get data file metadata")?
            .len()
            == 0
        {
            warn!(
                "no data exists at {}, creating new",
                config.data.to_string_lossy()
            );
            Self::new()
        } else {
            from_reader(BufReader::new(&file)).context("failed to deserialize data")?
        };

        Ok((data, file))
    }

    pub fn save_to_file(&self, file: &mut File) -> Result<()> {
        file.set_len(0)
            .context("failed to clear current data file")?;
        file.seek(SeekFrom::Start(0))
            .context("failed to go to start of file")?;

        let mut writer = BufWriter::new(file);

        if cfg!(debug_assertions) {
            to_writer_pretty(&mut writer, self, PrettyConfig::default())
        } else {
            to_writer(&mut writer, self)
        }
        .context("failed to serialize data")
    }
}
