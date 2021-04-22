use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::{BufReader, BufWriter, Seek, SeekFrom},
};

use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use fs2::FileExt;
use log::{debug, warn};
use ron::{
    de::from_reader,
    ser::{to_writer, to_writer_pretty, PrettyConfig},
};
use structopt::StructOpt;

mod completion;
mod config;
mod data;
mod emit;
mod ping;
mod speedtest;

use config::Config;
use data::Data;

#[derive(Debug, StructOpt)]
#[structopt(about = env!("CARGO_PKG_DESCRIPTION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Completion(completion::Command),
    Emit(emit::Command),
    Ping(ping::Command),
    SpeedTest(speedtest::Command),
}

#[tokio::main]
async fn main() -> Result<()> {
    human_panic::setup_panic!();
    env_logger::init();
    let opt = Opt::from_args();

    debug!("loading configuration");
    let config = load_config()?;
    debug!("loaded config: {:#?}", &config);

    debug!("loading data");
    let (mut data, mut file) = load_data(&config)?;
    debug!("loaded data: {:#?}", &data);

    debug!("executing command");
    let now = Utc::now();

    match opt.command {
        Command::Completion(c) => c.execute().context("failed to execute completion")?,
        Command::Emit(c) => c.execute().context("failed to execute emit")?,
        Command::Ping(c) => {
            let res = c
                .execute(&config.ping)
                .await
                .context("failed to execute ping")?;

            data.pings.insert(now, res);
        }
        Command::SpeedTest(s) => {
            let res = s
                .execute(&config.speed_test)
                .context("failed to execute speedtest")?;

            if let Some(r) = res {
                data.speed_tests.insert(now, r);
            }
        }
    }

    debug!("saving data");
    save_data(&data, &mut file)?;
    debug!("saved data");

    Ok(())
}

fn load_config() -> Result<Config> {
    let mut cfg = Figment::new().merge(Toml::file("/etc/nimo.toml"));
    if let Some(mut x) = home::home_dir() {
        x.push(".config/nimo.toml");
        cfg = cfg.merge(Toml::file(x));
    }
    cfg = cfg.merge(Env::prefixed("NIMO_").split("_"));
    cfg.extract().context("failed to load configuration")
}

fn load_data(config: &Config) -> Result<(Data, File)> {
    if let Some(x) = config.data.parent() {
        if x.is_file() {
            return Err(anyhow!("data directory is file"))?;
        } else if !x.exists() {
            create_dir_all(x).context("failed to create data directory")?;
        }
    }

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&config.data)
        .context("failed to open data file")?;
    file.lock_exclusive().context("failed to lock data file")?;

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
        Data::new()
    } else {
        from_reader(BufReader::new(&file)).context("failed to deserialize data")?
    };

    Ok((data, file))
}

fn save_data(data: &Data, file: &mut File) -> Result<()> {
    file.set_len(0)
        .context("failed to clear current data file")?;
    file.seek(SeekFrom::Start(0))
        .context("failed to go to start of file")?;

    let mut writer = BufWriter::new(file);

    if cfg!(debug_assertions) {
        to_writer_pretty(&mut writer, data, PrettyConfig::default())
    } else {
        to_writer(&mut writer, data)
    }
    .context("failed to serialize data")
}
