use std::{process::Command, os::unix::process::CommandExt};

pub fn install_deps(deps_command: &str, args: Vec<&str>) {

    let output = Command::new(deps_command).args(args).output();
    println!("Output-deps: {:?}", output)
}


pub fn execute_examples(execute: &str, args: Vec<String>) {
    println!("Execute: {}, Args: {:?}", execute, args);
    let output = Command::new(execute).args(args).output();
    println!("Output-examples: {:?}", output)
}
