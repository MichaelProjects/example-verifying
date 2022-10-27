mod language;
mod call;
mod command;

use std::{error::Error, env};
use std::fs;

use call::{install_deps, execute_examples};
use command::Examples;
use language::LanguageExample;
use structopt::StructOpt;



fn main() {
    let cli = Examples::from_args();
    let mut current_path = env::current_dir().expect("Could not get current path");
    let dir = cli.dir.clone();
    if !dir.contains(current_path.to_str().unwrap()) {
        current_path.push(dir);
            if !current_path.exists() {
                panic!("Directory does not exist")}
        };
    let examples = LanguageExample::new(current_path.to_str().unwrap().to_string(), cli.execute_command.clone(), cli.dependency_filename, cli.language_entry.clone(), cli.install_deps_command.clone());
    let deps = examples.get_dependencie_files().unwrap();
    let apps = examples.filter_executable().unwrap();
    println!("Deps: {:?}, Apps: {:?}", deps, apps);
    for x in deps {
        let file = x.as_os_str().to_str().unwrap();
        let command = cli.install_deps_command.split(" ").collect::<Vec<&str>>();

        let mut v = vec![];

        for j in command {
            v.push(j);
        }   
        v.push(file);

        let y = v.first().unwrap();
        println!("Command: {:?}, args: {:?}", y, v);
        install_deps(y.to_owned(), v);
    }

    for y in apps {
        execute_examples(&cli.language_entry, vec![y.as_os_str().to_str().unwrap().to_string()])
    }
}
