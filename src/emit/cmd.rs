use std::{
    fs::{self, write},
    io::BufWriter,
};

use anyhow::{Context, Result};
use include_dir::{include_dir, Dir};
use structopt::{clap::Shell, StructOpt};

static SYSTEMD: Dir = include_dir!("./systemd");

/// Emits some provided system configuration files into `/tmp`.
#[derive(Debug, structopt::StructOpt)]
pub struct Command {
    #[structopt(subcommand)]
    pub target: EmitTarget,
}

impl Command {
    pub fn execute(&self) -> Result<()> {
        let path = "/tmp/nimo";

        match self.target {
            EmitTarget::Completion { shell, stdout } => {
                let path = format!("{}/completion", path);
                fs::create_dir_all(&path).context("failed to create tmp dir")?;

                let mut output = Vec::<u8>::new();
                crate::Opt::clap().gen_completions_to(
                    "nimo",
                    shell,
                    &mut BufWriter::new(&mut output),
                );

                let output = String::from_utf8(output).context("failed to generate completions")?;

                if stdout {
                    println!("{}", output);
                } else {
                    write(
                        format!("{}/{}", &path, shell.to_string().to_lowercase()),
                        output,
                    )
                    .context("failed to write completion file")?;
                }
            }
            EmitTarget::Systemd => {
                let path = format!("{}/systemd", path);
                fs::create_dir_all(&path).context("failed to create tmp dir")?;

                for file in SYSTEMD.files() {
                    write(
                        format!("{}/{}", &path, file.path().display()),
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
    /// Emits a file containing shell completion information
    Completion {
        /// The shell to emit completion information for.
        ///
        /// For a list of supported shells, please refer to clap (https://clap.rs/).
        shell: Shell,
        /// Prints the completion to stdout instead of creating a file in `/tmp`.
        #[structopt(long)]
        stdout: bool,
    },
    /// Emits files relevant to systemd
    ///
    /// This will emit systemd service and timer files to run 'nimo ping' and 'nimo speed-test'
    /// periodically.
    Systemd,
}
