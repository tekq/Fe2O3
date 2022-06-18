use std::process::{Command, exit};
use std::env;
use std::path::Path;
use itertools::{concat, Itertools};
use url::Url;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let clone_args: Vec<String> = env::args().collect();


    if args.len() >= 2 { // detect action
            let action = &clone_args[1];

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
                args.remove(0); // remove exec name
                args.remove(0); // remove argument

                if args.len() == 1 {
                   println!("{0}ing: {1:?}", action, args);
                } else {
                    println!("{0}ing {1} packages: {2:?}", action, args.len(), args);
                }

                let mut package_to_install = 0;

                while package_to_install < args.len() {
                    println!("Installing package {0}/{1}", package_to_install + 1, args.len());
                    // println!("{}", "test".to_owned() + &args[package_to_install]);
                    let mut path = "/etc/elements/repos/Nitrogen/".to_owned() + &args[package_to_install];

                    if Path::new(&path).exists() {
                        println!("Wow! You exist!");
                        let exec = path.to_owned() + "/build";
                        Command::new("bash")
                            .arg(&exec)
                            .output()
                            .expect("Didn't work.");

                    // } else if {
                    //     TODO: use xbps w/ Void Repo if package not in default repo
                    } else {
                        println!("{0} is not in any repository.", args[package_to_install]);
                    }

                    // if Path::new(&path).exists() {
                    //     Command::new("bash")
                    //         .arg(exec)
                    //         .output()
                    //         .expect("Im outta here!");
                    // }

                    package_to_install = package_to_install + 1;
                    if package_to_install == args.len() {
                        exit(0);
                    }
                }

            } else {
                println!("No package specified to {0}.", action);
                exit(2);
            }


        }

        // Command::new("bash")
        //     .arg("/home/vlad/code/test")
        //     .output()
        //     .expect("Install command failed to start");
}
