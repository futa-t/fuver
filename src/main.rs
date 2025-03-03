use fuver::cli;

fn main() {
    if let Err(e) = cli::main() {
        eprintln!("{}", e);
    }
}
