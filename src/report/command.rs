use std::collections::BTreeMap;

use anyhow::Result;
use chrono::{DateTime, Duration, Utc};

#[derive(Debug, structopt::StructOpt)]
/// Prints a few simple evaluations to the command line.
pub struct Command {
    /// Classify ping tests either as success or failure instead of a spectrum.
    ///
    /// Without this option, the reliability is the ratio of received responses to sent requests.
    ///
    /// With this option, the reliability is the ratio of ping test where at least one target sent
    /// at least one response to all ping test.
    #[structopt(short, long)]
    clearcut: bool,
    /// Ignore all data not recorded within the last 7 days.
    #[structopt(short, long)]
    last_week: bool,
}

impl Command {
    pub fn execute(&self, data: &mut crate::Data) -> Result<()> {
        let after = if !self.last_week {
            None
        } else {
            Some(Utc::now() - Duration::days(7))
        };

        let reliability = match get_reliability(&data.pings, &after, self.clearcut) {
            Some(r) => format!("{:.3}%", r * 100.0),
            None => "no measurements".into(),
        };

        let (down, up) = match get_avg_speeds(&data.speed_tests, &after) {
            Some((down, up)) => (
                format!("{:.3} Mbps", down / 1_000_000.0 * 8.0),
                format!("{:.3} Mbps", up / 1_000_000.0 * 8.0),
            ),
            None => ("no measurements".into(), "no measurements".into()),
        };

        println!(
            "reliability:      {}\naverage download: {}\naverage upload:   {}",
            reliability, down, up
        );

        Ok(())
    }
}

fn get_reliability(
    data: &BTreeMap<DateTime<Utc>, crate::ping::Data>,
    after: &Option<DateTime<Utc>>,
    clearcut: bool,
) -> Option<f64> {
    let mut sent = 0;
    let mut received = 0;

    for (recorded, targets) in data {
        if !match after {
            Some(after) => recorded >= after,
            None => true,
        } {
            continue;
        }

        if !clearcut {
            for target in targets {
                sent += target.sent;
                received += target.received;
            }
        } else {
            sent += 1;

            for target in targets {
                if target.received > 0 {
                    received += 1;
                    break; // we only want to count one successfully pinged target, otherwise we
                           // would quickly reach success rates far above 100%.
                }
            }
        }
    }

    if sent > 0 {
        Some(received as f64 / sent as f64)
    } else {
        None
    }
}

fn get_avg_speeds(
    data: &BTreeMap<DateTime<Utc>, crate::speedtest::Data>,
    after: &Option<DateTime<Utc>>,
) -> Option<(f64, f64)> {
    let mut up = 0;
    let mut down = 0;
    let mut num = 0;

    for (recorded, test) in data {
        if !match after {
            Some(after) => recorded >= after,
            None => true,
        } {
            continue;
        };

        num += 1;
        up += test.upload;
        down += test.download;
    }

    if num == 0 {
        return None;
    }

    let num = num as f64;
    Some((down as f64 / num, up as f64 / num))
}
