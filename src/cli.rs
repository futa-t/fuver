use std::{
    env,
    fs::{self, File},
    path::PathBuf,
};

use crate::fuver::{self, FuVer, FuVerError};
use clap::Parser;
use std::str::FromStr;

const DEFAULT_FILE: &str = concat!(env!("CARGO_PKG_NAME"), ".toml");

#[derive(clap::Subcommand, Debug, Clone)]
enum VersionTarget {
    Major,
    Minor,
    Patch,
    Mask { pattern: String },
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
    All,
}

#[derive(clap::Subcommand, Debug)]
enum IncrementCommands {
    #[command(visible_alias = "ver")]
    Version {
        #[command(subcommand)]
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
    Version {
        #[command(subcommand)]
        target: Option<VersionTarget>,
    },
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
        file: String,
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
    config: Option<String>,

    #[command(subcommand)]
    cmd: Commands,
}

fn run_increment(cmd: IncrementCommands, fv: &mut FuVer) -> fuver::Result<()> {
    match cmd {
        IncrementCommands::Version { target } => match target {
            VersionTarget::Major => fv.incr_ver_major(),
            VersionTarget::Minor => fv.incr_ver_minor(),
            VersionTarget::Patch => fv.incr_ver_patch(),
            VersionTarget::Mask { pattern } => fv.incr_ver_mask(&pattern),
        },
        IncrementCommands::Major => fv.incr_ver_major(),
        IncrementCommands::Minor => fv.incr_ver_minor(),
        IncrementCommands::Patch => fv.incr_ver_patch(),

        IncrementCommands::PreRelease => fv.incr_pre(),

        IncrementCommands::BuildMetaData { target } => match target {
            Some(t) => match t {
                BuildMetaDataTarget::Number => fv.incr_build_num(),
                BuildMetaDataTarget::Date => fv.incr_build_date(),
                BuildMetaDataTarget::Hash => fv.incr_build_hash(),
                BuildMetaDataTarget::All => fv.incr_build_all(),
            },
            None => fv.incr_build_num(),
        },
    }?;
    Ok(())
}

fn run_init(file: &str) -> fuver::Result<()> {
    let p = PathBuf::from(file);
    if p.exists() {
        return Err(FuVerError::InitError("Already initialized.".to_string()));
    }
    File::create(&p).map_err(FuVerError::IO)?;
    let default = FuVer::default();
    let toml_str = toml::to_string(&default).map_err(|e| FuVerError::InitError(e.to_string()))?;
    fs::write(&p, toml_str).map_err(FuVerError::IO)?;
    println!("Initialize Success!");
    println!("file {}", p.to_string_lossy());
    println!("version {}", &default);
    Ok(())
}

pub fn main() -> fuver::Result<()> {
    let args = Args::parse();
    let conf_path = args.config.as_ref().unwrap();
    let file_str = fs::read_to_string(conf_path).map_err(FuVerError::IO)?;
    let mut c = FuVer::from_str(&file_str)?;

    match args.cmd {
        Commands::Init { file } => run_init(&file),
        Commands::Increment { silent, target } => run_increment(target, &mut c),
        Commands::Set { silent, target } => todo!(),
        Commands::Show { target } => {
            println!("{}", c.version);
            Ok(())
        }
    }?;
    c.save(conf_path);
    Ok(())
}
