use clap::Parser;

use fuver::BuildVerion;
use fuver::Version;

#[derive(Parser)]
#[command(author = "futa-t")]
struct Args {
    #[arg(short, long)]
    silent: bool,

    #[arg(long)]
    noincrement: bool,

    #[arg(short, long, default_value = ".fuver")]
    outdir: String,
}

fn main() {
    let args = Args::parse();

    let build = BuildVerion::new(&args.outdir);

    if !args.noincrement {
        build.increment().unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        });
    }

    if !args.silent {
        println!("{}", build);
    }
}
