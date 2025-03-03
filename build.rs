// use std::process::Command;
fn main() {
    // Lockの更新とかでも走っちゃうからPreCommitでCargo.toml更新するとコミット後に更新されちゃうんだよなぁ
    // _ = Command::new("fuver")
    //     .args(["incr", "-s", "build", "all"])
    //     .status();
}
