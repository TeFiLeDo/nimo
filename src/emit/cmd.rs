use std::fs::{self, write};

use anyhow::{Context, Result};
use include_dir::{include_dir, Dir};

static SYSTEMD: Dir = include_dir!("./systemd");

/// Emits some provided system configuration files into `/tmp`.
#[derive(Debug, structopt::StructOpt)]
pub struct Command {
    #[structopt(subcommand)]
    pub target: EmitTarget,
}

impl Command {
    pub fn execute(&self) -> Result<()> {
        match self.target {
            EmitTarget::Systemd => {
                let path = "/tmp/nimo/systemd/";
                fs::create_dir_all(path).context("failed to create tmp dir")?;

                for file in SYSTEMD.files() {
                    write(
                        format!("{}/{}", path, file.path().display()),
                        file.contents_utf8().unwrap(),
                    )
                    .context(format!(
                        r#"failed to write file "{}""#,
                        file.path().display()
                    ))?;
                }
            }
        };

        Ok(())
    }
}

#[derive(Debug, structopt::StructOpt)]
pub enum EmitTarget {
    /// Emit files relevant to systemd
    ///
    /// This will emit systemd service and timer files to run 'nimo ping' and 'nimo speed-test'
    /// periodically.
    Systemd,
}
