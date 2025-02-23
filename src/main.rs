use clap::{Parser, Subcommand};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::{io, str};

use fuver::*;

#[derive(Subcommand)]
enum SubCommands {
    Init,
    Show {
        #[command(subcommand)]
        command: Option<ShowCommands>,
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
        File::create(f)?;
    }
    Ok(())
}

fn run_cmd(cmd: SubCommands, c: &mut FuVer) {
    match cmd {
        SubCommands::Show { command } => match command {
            Some(cmd) => run_show_cmd(cmd, c),
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
