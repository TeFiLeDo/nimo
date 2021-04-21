use std::{convert::TryInto, time::Duration};

use anyhow::{anyhow, Context, Result};
use log::{debug, warn};
use surge_ping::Pinger;

/// Tests current internet connectivity utilizing pinging.
#[derive(Debug, structopt::StructOpt)]
pub struct Command {}

impl Command {
    pub async fn execute(self, config: &super::Config) -> Result<super::Data> {
        debug!("checking config");
        if config.count == 0 {
            return Err(anyhow!("number of pings cannot be 0"));
        }
        match config.targets.len() {
            0 => return Err(anyhow!("number of ping targets cannot be 0")),
            1 => warn!("it is recommended to specify two or more ping targets"),
            _ => {}
        };

        debug!("destructuring config");
        let count = config.count;
        let mut targets = Vec::with_capacity(config.targets.len());
        for (name, address) in &config.targets {
            targets.push((name.clone(), address.clone(), 0u8));
        }

        debug!("running ping");
        for (_name, address, counter) in &mut targets {
            let mut pinger = Pinger::new(address.clone()).context("failed to create pinger")?;
            pinger.timeout(Duration::from_secs(2));

            for i in 0..count {
                if pinger
                    .ping(i.try_into().context("failed to convert count to seq")?)
                    .await
                    .is_ok()
                {
                    *counter += 1
                }
            }
        }

        debug!("create results");
        let mut res = Vec::with_capacity(targets.len());
        for (target_name, target_address, received) in targets {
            res.push(super::data::DataEntry {
                target_name,
                target_address,
                sent: count,
                received,
            })
        }

        Ok(res)
    }
}
