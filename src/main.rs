use anyhow::{Context, Result};
use chrono::Utc;
use log::debug;
use structopt::StructOpt;

mod config;
mod data;
mod emit;
mod ping;
mod report;
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
    Emit(emit::Command),
    Ping(ping::Command),
    Report(report::Command),
    SpeedTest(speedtest::Command),
}

#[tokio::main]
async fn main() -> Result<()> {
    human_panic::setup_panic!();
    env_logger::init();
    let opt = Opt::from_args();

    debug!("loading configuration");
    let config = Config::load()?;
    debug!("loaded config: {:#?}", &config);

    if let Command::Emit(c) = opt.command {
        return c.execute().context("failed to execute emit");
    }

    let save = matches!(opt.command, Command::Ping(_) | Command::SpeedTest(_));

    debug!("loading data");
    let (mut data, mut file) = Data::load_from_file(&config, save)?;
    debug!("loaded data: {:#?}", &data);
    debug!("data {} be saved", if save { "will" } else { "will not" });

    debug!("executing command");
    let now = Utc::now();

    match opt.command {
        Command::Emit(_) => panic!("emit should already have exited"),
        Command::Ping(c) => {
            let res = c
                .execute(&config.ping)
                .await
                .context("failed to execute ping")?;

            data.pings.insert(now, res);
        }
        Command::Report(c) => c.execute(&data).context("failed to execute report")?,
        Command::SpeedTest(s) => {
            let res = s
                .execute(&config.speed_test)
                .context("failed to execute speedtest")?;

            if let Some(r) = res {
                data.speed_tests.insert(now, r);
            }
        }
    }

    if save {
        debug!("saving data");
        data.save_to_file(&mut file)?;
        debug!("saved data");
    } else {
        debug!("data was not modified");
    }

    Ok(())
}
