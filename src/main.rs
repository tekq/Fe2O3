use std::process::{Command, exit};
use std::io;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use nix::unistd::getuid;
use std::env;

fn open_package_database() -> std::io::Result<()> {
    let mut file = File::create("/tmp/temp")?;
    let buffer = "pkg";
    file.write_all(buffer.as_bytes())?;

    let mut input = File::open("/tmp/temp")?;
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer)?;
    Ok(())
}

fn main() {
    let mut args: Vec<String> = env::args().collect(); // take args in a vector
    let clone_args: Vec<String> = env::args().collect(); // have an imutable version of args

    if args.len() >= 2 { // detect action
        let action = &clone_args[1];

        if getuid().to_string().eq("0") {
            // pass
        } else {
            println!("You must be root to execute command: '{}'", args[1]);
            exit(128);
        }


        if action.to_lowercase().eq("install") { // detect action
                // pass
        } else if action.to_lowercase().eq("remove") {
                // pass
        } else if action.to_lowercase().eq("update") {
                if args.len() >= 3 {
                    println!("Updating Void packages 1/3");
                    Command::new("xbps-install")
                        .arg("-Suy")
                        .output()
                        .expect("Couldn't execute xbps");

                    println!("Removing old repository 2/3");
                    Command::new("rm")
                        .arg("-rf")// forced recursively remove
                        .arg("/etc/elements/repos/Nitrogen")// path to remove
                        .output()
                        .expect("Couldn't remove repository.");

                    println!("ReClone Repository 3/3");
                    Command::new("git")
                        .arg("clone")
                        .arg("https://github.com/NitrogenLinux/elements-repo.git")// Nitrogen Linux's main repository
                        .arg("/etc/elements/repos/Nitrogen")// path to clone to
                        .output()
                        .expect("Couldn't clone the repository.");

                    println!("Update complete: err_code: 0");
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
                   println!("{0}ing: {1:?}", action.to_lowercase(), args);
                } else {
                    println!("{0}ing {1} packages: {2:?}", action.to_lowercase(), args.len(), args);
                }

                let mut package_to_install = 0;

                while package_to_install < args.len() {
                    let path = "/etc/elements/repos/Nitrogen/".to_owned() + &args[package_to_install];

                    if Path::new(&path).exists() {
                        if action.to_string().eq("install"){
                            // ver =
                            // println!("Installing package {0}-{1} {2}/{3}", &args[package_to_install], ver, package_to_install + 1, args.len()); // print action and the number of packages remaining
                            Command::new("bash")
                                .arg(path.to_owned() + "/build")
                                .output()
                                .expect("Didn't work.");
                        } else if action.to_string().eq("remove") {
                            println!("Removing package {0} {1}/{2}", &args[package_to_install], package_to_install + 1, args.len()); // print action and the number of packages remaining
                            Command::new("bash")
                                .arg(path.to_owned() + "/remove")
                                .output()
                                .expect("Didn't work.");
                        } else if action.to_string().eq("update") {
                            println!("Updating package {0} {1}/{2}", &args[package_to_install], package_to_install + 1, args.len()); // print action and the number of packages remaining
                            Command::new("bash")
                                .arg(path.to_owned() + "/build")
                                .output()
                                .expect("Didn't work.");
                        }

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
                println!("No package specified to {0}.", action.to_lowercase());
                exit(2);
            }
        } else {
            println!("No command specified.");
    }
}
