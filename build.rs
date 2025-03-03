use std::process::Command;
fn main() {
    _ = Command::new("fuver")
        .args(["incr", "-s", "build", "all"])
        .status();
}
