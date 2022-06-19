use std::process::{Command, exit};
use std::env;
use std::path::Path;
use itertools::{concat, Itertools};
use std::fmt;
use nix::unistd::getuid;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let clone_args: Vec<String> = env::args().collect();

//    println!("{}", getuid());

    if args.len() >= 2 { // detect action
        let action = &clone_args[1];

        /* if getuid() != 0 {
             println!("You must be root to execute {}", args[1]);
            exit(128);
        } */


        if action.eq("install") { // detect action
                // pass
        } else if action.eq("remove") {
                // pass
        } else if action.eq("update") {
                if args.len() >= 3 {
                    Command::new("xbps-install")
                        .arg("-Suy")
                        .output()
                        .expect("Couldn't execute xbps");
                    exit(0);
                }
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
                    println!("{0}ing package {1}/{2}", action, package_to_install + 1, args.len());
                    // println!("{}", "test".to_owned() + &args[package_to_install]);
                    let mut path = "/etc/elements/repos/Nitrogen/".to_owned() + &args[package_to_install];

                    if Path::new(&path).exists() {
                        let exec = ""; // declare exec so compiler doesn't scream

                        if action.eq("install"){
                            let exec = path.to_owned() + "/build";
                        } else if action.eq("remove") {
                            let exec = path.to_owned() + "/remove";
                        } else if action.eq("update") {
                            let exec = path.to_owned() + "/build";
                        }


                        Command::new("bash")
                            .arg(&exec)
                            .output()
                            .expect("Didn't work.");

                    } else {
                        if action.eq("install") {
                            Command::new("xbps-install")
                                .arg("-Sy")
                                .arg(&args[package_to_install])
                                .output()
                                .expect("Couldn't execute xbps");
                        } else if action.eq("remove") {
                            Command::new("xbps-remove")
                                .arg("-y")
                                .arg(&args[package_to_install])
                                .output()
                                .expect("Couldn't execute xbps");

                        } else if action.eq("update") {
                            Command::new("xbps-install")
                                .arg("-Sy")
                                .arg(&args[package_to_install])
                                .output()
                                .expect("Couldn't execute xbps");
                        }
                    }

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
}
