use std::{process::Command, os::unix::process::CommandExt};

use log::{error};


pub fn install_deps(deps_command: &str, args: Vec<&str>) {
    let output = match Command::new(deps_command).args(args).output(){
    Ok(ok) => {
        let content = String::from_utf8(ok.stderr).unwrap();
            if content.contains("pip"){
                error!("Failed to execute app, {}", content)
            }
            return ()},
    Err(err) => {
        error!("Failed to execute example: {:?}", err);
            return ();
    }};
}


pub fn execute_examples(execute: &str, args: Vec<String>) {
    println!("Execute: {}, Args: {:?}", execute, args);
    match Command::new(execute).args(args).output(){
        Ok(ok) => {
            let content = String::from_utf8(ok.stderr).unwrap();
            if content.contains("Traceback"){
                error!("Failed to execute app, {}", content)
            }
            return ()},
        Err(err) => {
            error!("Failed to execute example: {:?}", err);
            return ();
        }
    };
}

#[test]
fn test_pip_install(){
    let res = Command::new("pip").output().unwrap();
    let mess = String::from_utf8(res.stdout).unwrap();
    println!("{}", mess);
    assert!(false)
}
