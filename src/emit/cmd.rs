use std::{
    fmt::Display,
    fs::{self, write},
    str::FromStr,
};

use anyhow::{Context, Result};
use include_dir::{include_dir, Dir};

static SYSTEMD: Dir = include_dir!("./systemd");

/// Emits some provided system configuration files into `/tmp`.
#[derive(Debug, structopt::StructOpt)]
pub struct Command {
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
    Systemd,
}

impl Display for EmitTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Systemd => "systemd",
        })
    }
}

impl FromStr for EmitTarget {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "systemd" => Self::Systemd,
            _ => return Err("unknown target".to_string()),
        })
    }
}
