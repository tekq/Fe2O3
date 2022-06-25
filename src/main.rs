use std::process::{Command, exit};
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use nix::unistd::getuid;
use std::env;

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
            // pass
        } else if action.to_lowercase().eq("search") {
            // pass
        } else {
            println!("No argument called '{0}' found.", action);
            exit(1);
        }

        if args.len() >= 3 { // detect if package is specified and install
            args.remove(0); // remove exec name
            args.remove(0); // remove argument

            if args.len() == 1 {
                if action.to_lowercase().eq("search") {
                    // search
                    if Path::new(&("/etc/elements/repos/Nitrogen/".to_owned() + &args[0])).exists() {
                        println!("Found {}", args[0]);
                    } else {
                        println!("No package called '{0}' found.", args[0]);
                        exit(1);
                    }
                    exit(0);
                }

                if action.to_lowercase().eq("install") {
                    println!("Installing: {0:?}", args);
                } else if action.to_lowercase().eq("remove") {
                    println!("Removing: {0:?}", args);
                } else if action.to_lowercase().eq("update") {
                    println!("Updating: {0:?}", args);
                }
            } else {
                if action.to_lowercase().eq("install") {
                    println!("Installing {0} packages: {1:?}", args.len(), args);
                } else if action.to_lowercase().eq("remove") {
                    println!("Removing {0} packages: {1:?}", args.len(), args);
                } else if action.to_lowercase().eq("update") {
                    println!("Updating {0} packages: {1:?}", args.len(), args);
                }
            }

            let mut package_to_install = 0;

            while package_to_install < args.len() {
                let mut pkg_db_path = File::open("/etc/elements/.pkg.db").unwrap();
                let mut updated_pkg_db = String::new();
                pkg_db_path.read_to_string(&mut updated_pkg_db).unwrap();

                let path = "/etc/elements/repos/Nitrogen/".to_owned() + &args[package_to_install];

                if Path::new(&path).exists() {
                    if action.to_string().eq("install") {
                        if updated_pkg_db.contains(&args[package_to_install]) {
                            println!("{} already installed. Reinstalling.", &args[package_to_install]);
                        } else {
                            let updated_pkg_db = updated_pkg_db + &*args[package_to_install] + " ";
                            write_to_package_db(updated_pkg_db);
                        }

                        Command::new("bash")
                            .arg(path.to_owned() + "/build")
                            .output()
                            .expect("Didn't work.");
                    } else if action.to_string().eq("remove") {
                        if !updated_pkg_db.contains(&args[package_to_install]) {
                            println!("Cannot remove {}: Package not installed.", &args[package_to_install]);
                            exit(256);
                        } else {
                            let updated_pkg_db = updated_pkg_db.replace(&args[package_to_install], "");
                            write_to_package_db(updated_pkg_db);
                        }

                        println!("Removing package {0} {1}/{2}", &args[package_to_install], package_to_install + 1, args.len()); // print action and the number of packages remaining
                        Command::new("bash")
                            .arg(path.to_owned() + "/remove")
                            .output()
                            .expect("Didn't work.");
                    } else if action.to_string().eq("update") {
                        if !updated_pkg_db.contains(&args[package_to_install]) {
                            println!("Cannot update {}: Package not installed.", &args[package_to_install]);
                            exit(256);
                        }
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
                    } else {
                        println!("{} is not a valid action.", action);
                        exit(1);
                    }
                    exit(0);
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
    }
}


fn write_to_package_db(package: String) -> std::io::Result<()> {
    let mut package_db = File::create("/etc/elements/.pkg.db").unwrap();
    package_db.write_all(package.as_bytes()).expect("write failed");

    let mut input = File::open("/tmp/temp")?;
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer)?;
    Ok(())
}