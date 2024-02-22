use std::process::Command;

fn main() {
    print_header();
}

fn print_header() {
    let mut bash = Command::new("bash");

    bash.arg("./util/header.sh");
    bash.status().expect("process failed to execute");
}
