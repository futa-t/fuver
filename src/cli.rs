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
enum IncrVersionTarget {
    Major,
    Minor,
    Patch,
    Mask { pattern: String },
}

#[derive(clap::Subcommand, Debug, Clone)]
enum ShowVersionTarget {
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
    All,
}

#[derive(clap::Subcommand, Debug, Clone)]
enum SetBuildMetaDataTarget {
    Number { value: usize },
    Date { value: String },
    Hash { value: String },
    Format { value: String },
}

#[derive(clap::Subcommand, Debug)]
enum IncrementCommands {
    #[command(visible_alias = "ver")]
    Version {
        #[command(subcommand)]
        target: IncrVersionTarget,
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
    Version {
        version: String,
    },
    Major {
        version: usize,
    },
    Minor {
        version: usize,
    },
    Patch {
        version: usize,
    },
    #[command(visible_alias = "pre")]
    PreRelease {
        tag: String,
        number: Option<usize>,
    },
    #[command(visible_alias = "build")]
    BuildMetaData {
        #[command(subcommand)]
        target: SetBuildMetaDataTarget,
    },
    Date {
        value: String,
    },
    Hash {
        value: String,
    },
}

#[derive(clap::Subcommand, Debug)]
enum ShowCommands {
    #[command(visible_alias = "ver")]
    Version {
        #[command(subcommand)]
        target: Option<ShowVersionTarget>,
    },
    Major,
    Minor,
    Patch,
    #[command(visible_alias = "pre")]
    PreRelease {
        target: Option<PreReleaseTarget>,
    },
    #[command(visible_alias = "build")]
    BuildMetaData {
        target: Option<BuildMetaDataTarget>,
        #[arg(short, long)]
        format: Option<String>,
    },
    Date,
    Hash,
    Full,
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

fn run_increment(fv: &mut FuVer, cmd: IncrementCommands, silent: bool) -> fuver::Result<()> {
    match cmd {
        IncrementCommands::Version { target } => match target {
            IncrVersionTarget::Major => fv.incr_ver_major(silent),
            IncrVersionTarget::Minor => fv.incr_ver_minor(silent),
            IncrVersionTarget::Patch => fv.incr_ver_patch(silent),
            IncrVersionTarget::Mask { pattern } => fv.incr_ver_mask(&pattern, silent),
        },
        IncrementCommands::Major => fv.incr_ver_major(silent),
        IncrementCommands::Minor => fv.incr_ver_minor(silent),
        IncrementCommands::Patch => fv.incr_ver_patch(silent),

        IncrementCommands::PreRelease => fv.incr_pre(silent),

        IncrementCommands::BuildMetaData { target } => match target {
            Some(t) => match t {
                BuildMetaDataTarget::Number => fv.incr_build_num(silent),
                BuildMetaDataTarget::Date => fv.incr_build_date(silent),
                BuildMetaDataTarget::Hash => fv.incr_build_hash(silent),
                BuildMetaDataTarget::All => fv.incr_build_all(silent),
            },
            None => fv.incr_build_num(silent),
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

fn run_show(fv: &FuVer, cmd: ShowCommands) -> fuver::Result<()> {
    match cmd {
        ShowCommands::Version { target } => match target {
            Some(ShowVersionTarget::Major) => fv.show_major(),
            Some(ShowVersionTarget::Minor) => fv.show_minor(),
            Some(ShowVersionTarget::Patch) => fv.show_patch(),
            None => fv.show_version(),
        },
        ShowCommands::Major => fv.show_major(),
        ShowCommands::Minor => fv.show_minor(),
        ShowCommands::Patch => fv.show_patch(),
        ShowCommands::PreRelease { target } => match target {
            Some(t) => match t {
                PreReleaseTarget::Tag => fv.show_prerelease_tag(),
                PreReleaseTarget::Number => fv.show_prerelease_number(),
            },
            None => fv.show_prerelease(),
        },
        ShowCommands::BuildMetaData { target, format } => {
            if let Some(fmt) = format {
                return fv.show_build_fmt(&fmt);
            }
            match target {
                Some(t) => match t {
                    BuildMetaDataTarget::Number => fv.show_build_number(),
                    BuildMetaDataTarget::Date => fv.show_build_date(),
                    BuildMetaDataTarget::Hash => fv.show_build_hash(),
                    BuildMetaDataTarget::All => fv.show_build_all(),
                },
                None => fv.show_build(),
            }
        }
        ShowCommands::Date => fv.show_build_date(),
        ShowCommands::Hash => fv.show_build_hash(),
        ShowCommands::Full => fv.show_full(),
    }?;
    Ok(())
}

fn run_set(fv: &mut FuVer, cmd: SetCommands, silent: bool) -> fuver::Result<()> {
    match cmd {
        SetCommands::Version { version } => fv.set_version(&version, silent),
        SetCommands::Major { version } => fv.set_major(version, silent),
        SetCommands::Minor { version } => fv.set_minor(version, silent),
        SetCommands::Patch { version } => fv.set_patch(version, silent),
        SetCommands::PreRelease { tag, number } => fv.set_pre(&tag, number, silent),
        SetCommands::BuildMetaData { target } => match target {
            SetBuildMetaDataTarget::Number { value } => fv.set_build_number(value, silent),
            SetBuildMetaDataTarget::Date { value } => fv.set_build_date(&value, silent),
            SetBuildMetaDataTarget::Hash { value } => fv.set_build_hash(&value, silent),
            SetBuildMetaDataTarget::Format { value } => fv.set_build_fmt(&value, silent),
        },
        SetCommands::Date { value } => fv.set_build_date(&value, silent),
        SetCommands::Hash { value } => fv.set_build_hash(&value, silent),
    }
}

pub fn main() -> fuver::Result<()> {
    let args = Args::parse();
    let conf_path = args.config.as_ref().unwrap();
    let file_str = fs::read_to_string(conf_path).map_err(FuVerError::IO)?;
    let mut fv = FuVer::from_str(&file_str)?;

    match args.cmd {
        Commands::Init { file } => run_init(&file),
        Commands::Increment { silent, target } => run_increment(&mut fv, target, silent),
        Commands::Set { silent, target } => run_set(&mut fv, target, silent),
        Commands::Show { target } => {
            match target {
                Some(cmd) => run_show(&fv, cmd),
                None => fv.show_version(),
            }?;
            Ok(())
        }
    }?;
    fv.save(conf_path);
    Ok(())
}
