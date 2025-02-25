// use clap::{Parser, Subcommand};
// use std::fs::{self, File};
// use std::path::{Path, PathBuf};
// use std::{io, str};

// #[derive(Subcommand)]
// enum SubCommands {
//     Init,
//     Show {
//         #[command(subcommand)]
//         command: Option<ShowCommands>,
//     },
//     Set {
//         #[command(subcommand)]
//         command: SetCommands,
//     },
//     Remove {
//         target: RemoveTarget,
//     },
//     Increment {
//         #[command(subcommand)]
//         command: Option<IncrementCommands>,
//         // #[arg(value_enum)]
//         // what: IncrementType,
//         // #[arg(default_value = "x.x.x")]
//         // value: String,
//         // #[arg(short, long)]
//         // silent: bool,
//     },
// }

// #[derive(Subcommand)]
// pub enum SetCommands {
//     Version,
//     Pre {
//         tag: String,
//         number: Option<usize>,
//         #[arg(short, long)]
//         silent: bool,
//     },
//     Build,
// }

// #[derive(clap::ValueEnum, Clone)]
// enum RemoveTarget {
//     Pre,
// }
// #[derive(clap::ValueEnum, Clone)]
// enum IncrementType {
//     Version,
//     Build,
// }

// #[derive(Parser)]
// #[command(author = "futa-t")]
// struct Args {
//     #[command(subcommand)]
//     command: SubCommands,
// }

// fn init(f: &Path) -> io::Result<()> {
//     if f.exists() {
//         println!("すでに初期化済みです")
//     } else {
//         File::create(f)?;
//     }
//     Ok(())
// }

// fn run_set_cmd(cmd: SetCommands, c: &mut FuVer) {
//     match cmd {
//         SetCommands::Version => {}
//         SetCommands::Pre {
//             tag,
//             number,
//             silent,
//         } => {
//             let _ = c.set_prerelease(&tag, number);
//             if !silent {
//                 c.show_prerelease();
//             }
//         }
//         SetCommands::Build => todo!(),
//     }
// }
// fn run_cmd(cmd: SubCommands, c: &mut FuVer) {
//     match cmd {
//         SubCommands::Show { command } => match command {
//             Some(cmd) => match run_show_cmd(cmd, c) {
//                 Ok(()) => {}
//                 Err(e) => eprintln!("{}", e),
//             },
//             None => println!("{}", c),
//         },
//         SubCommands::Set { command } => run_set_cmd(command, c),
//         SubCommands::Increment { command } => {
//             match command {
//                 Some(cmd) => match run_increment(cmd, c) {
//                     Ok(_) => {}
//                     Err(e) => eprintln!("{}", e),
//                 },

//                 None => println!("{}", c),
//             }
//             //     In::Version => {
//             //         if let Err(e) = c.increment_version(&value) {
//             //             eprintln!("{}", e);
//             //         }
//             //     }
//             //     IncrementType::Build => {
//             //         if let Err(e) = c.increment_build() {
//             //             eprintln!("{}", e)
//             //         }
//             //     }
//             // }
//             // if !silent {
//             //     println!("{}", c);
//             // }
//         }
//         SubCommands::Init => {}
//         SubCommands::Remove { target } => match target {
//             RemoveTarget::Pre => c.pre = None,
//         },
//     }
// }

use fuver::cli;

fn main() {
    if let Err(e) = cli::main() {
        eprintln!("{}", e);
    }
}
