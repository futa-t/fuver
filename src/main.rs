use clap::{Parser, Subcommand};
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};

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
    #[command(subcommand)]
    command: SubCommands,
}

fn init(f: &Path) -> io::Result<()> {
    if f.exists() {
        println!("すでに初期化済みです")
    } else {
        File::create(&f)?;
    }
    Ok(())
}

fn run_cmd(cmd: SubCommands, c: &mut FuVer) {
    match cmd {
        SubCommands::Show { what } => match what {
            Some(ShowType::Version) => c.show_version(),
            Some(ShowType::Build) => c.show_build(),
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
        SubCommands::Init => {}
    }
}

fn main() {
    let args = Args::parse();
    let conf = PathBuf::from("fuver.toml");

    match args.command {
        SubCommands::Init => {
            init(&conf).unwrap();
        }
        cmd => {
            if !conf.exists() {
                eprintln!("fuver.tomlがみつかりませんでした。fuver init を実行してください");
                std::process::exit(1);
            }
            let cs = fs::read_to_string(&conf).unwrap();
            let mut c: FuVer = toml::from_str(&cs).unwrap();
            run_cmd(cmd, &mut c);
            c.save(&conf);
        }
    }
}
