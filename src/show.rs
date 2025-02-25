// use clap::Subcommand;

// use crate::FuVer;

// #[derive(Subcommand, Clone)]
// pub enum ShowCommands {
//     Version,
//     Pre,
//     Build {
//         #[command(subcommand)]
//         command: Option<ShowBuildCommands>,
//         #[arg(
//             short,
//             long,
//             help = "{number|num|n} {date|d(:strftime format)} {hash|h(:digits)}"
//         )]
//         format: Option<String>,
//     },
// }

// #[derive(Subcommand, Clone)]
// #[command(
//     about = "Print BuildMetaData.",
//     long_about = "Print BuildMetaData in a given format.\n\
//     format is {number|num|n} {date|d(:strftime format)} {hash|h(:digits)}\n\
//     If no format is specified, display +build.{number}.{date:%Y%m%d}"
// )]
// pub enum ShowBuildCommands {
//     #[command(about = "[alias] --format \"+build.{number}\"")]
//     Number,
//     #[command(about = "[alias] --format \"+date.{%Y%m%d}\"")]
//     Date,
//     #[command(about = "[alias] --format \"+hash.{hash:8}\"")]
//     Hash,
// }

// pub fn run_show_cmd(command: ShowCommands, c: &mut FuVer) -> Result<(), String> {
//     match command {
//         ShowCommands::Version => c.show_version(),
//         ShowCommands::Pre => c.show_prerelease(),
//         ShowCommands::Build { command, format } => {
//             if let Some(fmt) = format {
//                 return c.build.show_fmt(&fmt).map_err(|e| e.to_string());
//             }
//             match command {
//                 Some(ShowBuildCommands::Number) => c.build.show_fmt("build.{number}"),
//                 Some(ShowBuildCommands::Date) => c.build.show_fmt("date.{date:%Y%m%d}"),
//                 Some(ShowBuildCommands::Hash) => c.build.show_fmt("hash.{hash:8}"),
//                 None => c.build.show(),
//             }
//             .map_err(|e| e.to_string())?
//         }
//     }
//     Ok(())
// }
