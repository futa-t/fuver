use clap::Subcommand;

use crate::FuVer;

#[derive(Subcommand)]
pub enum IncrementCommands {
    Version,
    Build {
        #[command(subcommand)]
        command: Option<IncrementBuildCommands>,
    },
}

#[derive(Subcommand, Clone)]
pub enum IncrementBuildCommands {
    Number,
    Date,
    Hash,
}

pub fn run_increment(cmd: IncrementCommands, c: &mut FuVer) {
    let _ = match cmd {
        IncrementCommands::Version => c.increment_version("x.x.1"),
        IncrementCommands::Build { command } => match command {
            Some(IncrementBuildCommands::Number) => c.build.increment_number(),
            Some(IncrementBuildCommands::Date) => c.build.update_date(),
            Some(IncrementBuildCommands::Hash) => c.build.update_hash(),
            None => Ok(()),
        }
        .map_err(|e| e.to_string()),
    };
}
