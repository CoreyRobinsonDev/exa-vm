use std::process::Command;

use exa_vm::Exa;

fn main() {
    print_header();

    let xa = Exa::new("exapunks/XA.exa".to_string());

    println!("{:?}", xa);
}

fn print_header() {
    let mut bash = Command::new("bash");
    let mut clear = Command::new("clear");

    clear.status().expect("process failed to execute");
    bash.arg("./util/exapunks.sh");
    bash.status().expect("process failed to execute");
}
