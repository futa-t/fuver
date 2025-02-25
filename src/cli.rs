use std::{env, path::PathBuf};

use clap::Parser;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_FILE: &str = concat!(env!("CARGO_PKG_NAME"), ".toml");

#[derive(clap::ValueEnum, Debug, Clone)]
enum VersionTarget {
    Major,
    Minor,
    Patch,
}

#[derive(clap::ValueEnum, Debug, Clone)]
enum PreReleaseTarget {
    Tag,
    Number,
}

#[derive(clap::ValueEnum, Debug, Clone)]
enum BuildMetaDataTarget {
    Number,
    Date,
    Hash,
}

#[derive(clap::Subcommand, Debug)]
enum IncrementCommands {
    #[command(visible_alias = "ver")]
    Version {
        target: VersionTarget,
    },
    Major,
    Minor,
    Patch,
    #[command(visible_alias = "pre")]
    PreRelease,
    #[command(visible_alias = "build")]
    BuildMetaData {
        target: Option<BuildMetaDataTarget>,
    },
}

#[derive(clap::Subcommand, Debug)]
enum SetCommands {
    #[command(visible_alias = "ver")]
    Version { version: String },
    #[command(visible_alias = "pre")]
    PreRelease { tag: String, number: Option<usize> },
    #[command(visible_alias = "build")]
    BuildMetaData { format: String },
}

#[derive(clap::Subcommand, Debug)]
enum ShowCommands {
    #[command(visible_alias = "ver")]
    Version { target: Option<VersionTarget> },
    #[command(visible_alias = "pre")]
    PreRelease { target: Option<PreReleaseTarget> },
    #[command(visible_alias = "build")]
    BuildMetaData {
        target: Option<BuildMetaDataTarget>,
        #[arg(short, long)]
        format: Option<String>,
    },
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    Init {
        #[arg(default_value = DEFAULT_FILE)]
        config: Option<PathBuf>,
    },
    #[command(visible_alias = "incr")]
    Increment {
        #[arg(short, long)]
        silent: bool,
        #[command(subcommand)]
        target: IncrementCommands,
    },
    Set {
        #[arg(short, long)]
        silent: bool,
        #[command(subcommand)]
        target: SetCommands,
    },
    Show {
        #[command(subcommand)]
        target: Option<ShowCommands>,
    },
}

#[derive(clap::Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value = DEFAULT_FILE)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    cmd: Commands,
}

impl Args {
    pub fn show(&self) {
        println!("{:#?}", self);
    }
}

pub fn main() {
    let args = Args::parse();
    println!("fuver {}", VERSION);
    args.show();
}
