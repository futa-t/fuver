use clap::Parser;
use std::fs::*;
use std::io::*;

#[derive(Parser)]
#[command(author = "futa-t")]
struct Args {
    #[arg(short, long)]
    silent: bool,

    #[arg(long)]
    noincrement: bool,
}
fn increment_build_number() -> usize {
    let f_build = "buildversion";

    let mut version = String::new();

    match File::open(f_build) {
        Ok(mut f) => {
            let r = f.read_to_string(&mut version);
            if let Err(_) = r {
                version = "0".to_string();
            }
        }
        Err(_) => {
            version = "0".to_string();
        }
    }

    let mut version = version.parse::<usize>().unwrap_or(0usize);
    version += 1;

    match File::create(f_build) {
        Ok(f) => {
            let mut bf = BufWriter::new(f);
            bf.write(version.to_string().as_bytes()).unwrap();
        }
        Err(e) => println!("{}", e.to_string()),
    }
    return version;
}

fn get_current_build() -> usize {
    let f_build = "buildversion";

    let mut version = String::new();

    match File::open(f_build) {
        Ok(mut f) => {
            let r = f.read_to_string(&mut version);
            if let Err(_) = r {
                return 0;
            }
        }
        Err(_) => {
            return 0;
        }
    }

    version.parse::<usize>().unwrap_or(0usize)
}

fn main() {
    let args = Args::parse();

    if args.noincrement {
        println!("{}", get_current_build());
        return;
    }

    let build_number = increment_build_number();
    if !args.silent {
        println!("{}", build_number);
    }
}
