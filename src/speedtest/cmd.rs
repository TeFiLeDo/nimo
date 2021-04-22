use std::{io::ErrorKind::NotFound, process::Command as Cmd};

use anyhow::{anyhow, Context, Result};
use log::{debug, error};
use serde_json::{from_slice, Value};

const DT: &'static str = "invalid data type";
const CT: &'static str = "failed to parse speedtest output";

/// Tests current internet speed utilizing speedtest.net.
#[derive(Debug, structopt::StructOpt)]
pub struct Command {}

impl Command {
    pub fn execute(self, config: &super::Config) -> Result<Option<super::Data>> {
        debug!("checking  config");
        if !config.enabled {
            error!("speedtest not enabled, exiting gracefully");
            return Ok(None);
        }

        debug!("running test");
        let output = match Cmd::new("speedtest").arg("--format").arg("json").output() {
            Ok(x) => x,
            Err(e) => {
                if let NotFound = e.kind() {
                    return Err(anyhow!(r#""speedtest" command not found"#));
                } else {
                    return Err(e).context("failed to run speedtest cli");
                }
            }
        };

        debug!("deserializing output");
        let val: Value =
            from_slice(&output.stdout).context("failed to parse deserialize output")?;

        debug!("parsing output");
        if let Some(str) = val["error"].as_str() {
            return Err(anyhow!(str.to_owned())).context("failed to run speed test");
        }

        // server
        Ok(Some(super::Data {
            server_id: val["server"]["id"]
                .as_u64()
                .ok_or(anyhow!(DT))
                .context(CT)?,
            server_host: val["server"]["host"]
                .as_str()
                .ok_or(anyhow!(DT))
                .context(CT)?
                .to_owned(),
            server_name: val["server"]["name"]
                .as_str()
                .ok_or(anyhow!(DT))
                .context(CT)?
                .to_owned(),
            server_country: val["server"]["country"]
                .as_str()
                .ok_or(anyhow!(DT))
                .context(CT)?
                .to_owned(),
            isp: val["isp"]
                .as_str()
                .ok_or(anyhow!(DT))
                .context(CT)?
                .to_owned(),
            upload: val["upload"]["bandwidth"]
                .as_u64()
                .ok_or(anyhow!(DT))
                .context(CT)?,
            download: val["download"]["bandwidth"]
                .as_u64()
                .ok_or(anyhow!(DT))
                .context(CT)?,
        }))
    }
}
