use std::collections::BTreeMap;

use anyhow::Result;
use chrono::{DateTime, Utc};

#[derive(Debug, structopt::StructOpt)]
pub struct Command {
    /// Classify ping tests either as success or failure instead of a spectrum.
    ///
    /// Without this option, the reliability is the ratio of received responses to sent requests.
    ///
    /// With this option, the reliability is the ratio of ping test where at least one target sent
    /// at least one response to all ping test.
    #[structopt(short, long)]
    clearcut: bool,
}

impl Command {
    pub fn execute(&self, data: &mut crate::Data) -> Result<()> {
        let reliability = match if !self.clearcut {
            get_reliability(&data.pings)
        } else {
            get_clearcut_reliability(&data.pings)
        } {
            Some(r) => format!("{}%", r * 100.0),
            None => "no measurements".into(),
        };

        let (down, up) = match get_avg_speeds(&data.speed_tests) {
            Some((down, up)) => (down.to_string(), up.to_string()),
            None => ("no measurements".into(), "no measurements".into()),
        };

        println!(
            "reliability:      {}\naverage download: {}\naverage upload:   {}",
            reliability, down, up
        );

        Ok(())
    }
}

fn get_reliability(data: &BTreeMap<DateTime<Utc>, crate::ping::Data>) -> Option<f64> {
    let mut sent = 0;
    let mut received = 0;

    for (_, targets) in data {
        for target in targets {
            sent += target.sent;
            received += target.received
        }
    }

    if sent > 0 {
        Some(received as f64 / sent as f64)
    } else {
        None
    }
}

fn get_clearcut_reliability(data: &BTreeMap<DateTime<Utc>, crate::ping::Data>) -> Option<f64> {
    let mut successes = 0;

    'd: for (_, targets) in data {
        for target in targets {
            if target.received > 0 {
                successes += 1;
                continue 'd;
            }
        }
    }

    if data.len() > 0 {
        Some(successes as f64 / data.len() as f64)
    } else {
        None
    }
}

fn get_avg_speeds(data: &BTreeMap<DateTime<Utc>, crate::speedtest::Data>) -> Option<(f64, f64)> {
    if data.len() == 0 {
        return None;
    }

    let mut up = 0;
    let mut down = 0;

    for (_, test) in data {
        up += test.upload;
        down += test.download;
    }

    let num = data.len() as f64;
    Some((down as f64 / num, up as f64 / num))
}
