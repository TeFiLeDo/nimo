use anyhow::Result;
use structopt::{clap::Shell, StructOpt};

/// Generates command completion files for some supported shells.
#[derive(Debug, StructOpt)]
pub struct Command {
    shell: Shell,
}

impl Command {
    pub fn execute(&self) -> Result<()> {
        crate::Opt::clap().gen_completions_to("nimo", self.shell, &mut std::io::stdout());

        Ok(())
    }
}
