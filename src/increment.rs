use clap::Subcommand;

use crate::{version, FuVer};

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum VersionTarget {
    Major,
    Minor,
    Patch,
    Mask,
}

#[derive(Subcommand)]
pub enum IncrementCommands {
    Version {
        target: VersionTarget,
        #[clap(required_if_eq("target", "mask"))]
        mask: Option<String>,
    },
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

pub fn run_increment(cmd: IncrementCommands, c: &mut FuVer) -> Result<(), String> {
    match cmd {
        IncrementCommands::Version { target, mask } => {
            match target {
                VersionTarget::Major => c.version.increment_major(),
                VersionTarget::Minor => c.version.increment_minor(),
                VersionTarget::Patch => c.version.increment_patch(),
                VersionTarget::Mask => match mask {
                    Some(m) => c.version.increment_mask(&m),
                    None => Err(version::VersionError::Format(
                        "x.y.z形式のマスクを指定してください".to_string(),
                    )),
                },
            }
        }
        .map_err(|e| e.to_string()),
        IncrementCommands::Build { command } => match command {
            Some(IncrementBuildCommands::Number) => c.build.increment_number(),
            Some(IncrementBuildCommands::Date) => c.build.update_date(),
            Some(IncrementBuildCommands::Hash) => c.build.update_hash(),
            None => Ok(()),
        }
        .map_err(|e| e.to_string()),
    }
}
