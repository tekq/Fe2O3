use std::process::{Command, exit};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();



    if args.len() >= 2 { // detect action
        let action = &args[1];

        if action.eq("install") { // detect action
            // pass
        } else if action.eq("remove") {
            // pass
        } else if action.eq("update") {
            // pass
        } else {
            println!("No argument called '{0}' found.", action);
            exit(1);
        }

        if args.len() >= 3 { // detect if package is specified and install
            let pkgs = &args[2];
            println!("{0}ing {1} packages: {2:?}", action, pkgs.len() - 6, pkgs);
        } else {
            println!("No package specified to {0}.", action);
            exit(2);
        }


    }

    Command::new("bash")
        .arg("/home/vlad/code/test")
        .output()
        .expect("Install command failed to start");

    let err_code = 0;

    println!("Installation finished with code {}", err_code);
}
