use clap::Parser;
use std::fs::*;
use std::io::*;

#[derive(Parser)]
#[command(author = "futa-t")]
struct Args {
    #[arg(short = 's', long = "silent")]
    silent: bool,
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

fn main() {
    let args = Args::parse();
    if !args.silent {
        println!("{}", increment_build_number());
    }
}
