use clap::{Parser, Subcommand};
use std::fs::{self, File};
use std::io;
use std::path::PathBuf;

use fuver::*;

#[derive(Subcommand)]
enum SubCommands {
    Init,
    Show {
        #[arg(value_enum)]
        what: Option<ShowType>,
    },
    Increment {
        #[arg(value_enum)]
        what: IncrementType,
        #[arg(default_value = "x.x.x")]
        value: String,
        #[arg(short, long)]
        silent: bool,
    },
}

#[derive(clap::ValueEnum, Clone)]
enum ShowType {
    Version,
    Build,
}

#[derive(clap::ValueEnum, Clone)]
enum IncrementType {
    Version,
    Build,
}

#[derive(Parser)]
#[command(author = "futa-t")]
struct Args {
    #[arg(short, long, default_value = "fuver.toml")]
    config: PathBuf,

    #[command(subcommand)]
    command: SubCommands,
}

fn init() -> io::Result<()> {
    let f = PathBuf::from("fuver.toml");
    if f.exists() {
        println!("すでに初期化済みです");
    } else {
        File::create(f)?;
    }
    Ok(())
}

fn main() {
    let args = Args::parse();

    if !args.config.exists() {
        File::create(&args.config).unwrap();
    }
    let cs = fs::read_to_string(&args.config).unwrap();
    let mut c: FuVer = toml::from_str(&cs).unwrap();
    // println!("current version: {}", c);

    match args.command {
        SubCommands::Init => init().unwrap(),
        SubCommands::Show { what } => match what {
            Some(ShowType::Version) => println!("{}", c.version),
            Some(ShowType::Build) => println!("{}", c.build),
            None => println!("{}", c),
        },
        SubCommands::Increment {
            what,
            value,
            silent,
        } => {
            match what {
                IncrementType::Version => {
                    if let Err(e) = c.increment_version(&value) {
                        eprintln!("{}", e);
                    }
                }
                IncrementType::Build => {
                    if let Err(e) = c.increment_build() {
                        eprintln!("{}", e)
                    }
                }
            }
            if !silent {
                println!("{}", c);
            }
        }
    }

    c.save(&args.config);
}
