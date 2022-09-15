/*
 * Author: Dylan Turner
 * Description: Entry point for one-pkg project
 */

mod args;

use crate::args::get_args;

fn main() {
    let command = get_args();
    let sub_cmd = command.subcommand();
    if sub_cmd.is_some() {
        let (name, args) = sub_cmd.unwrap();
        match name {
            "install" => install(args.value_of("PACKAGE").expect("No package name provided!")),
            "uninstall" => uninstall(args.value_of("PACKAGE").expect("No package name provided!")),
            "auto-uninstall" => auto_uninstall(),
            _ => {} // Handled by clap
    
        }
    } else {
        println!("No command provided. Don't know how to use? You probably want to add --help.");
    }
}

fn install(pkg_name: &str) {
    println!("Installing package {}.", pkg_name);
}

fn uninstall(pkg_name: &str) {
    println!("Uninstalling package {}.", pkg_name);
}

fn auto_uninstall() {
    println!("Auto-uninstalling packages.");
}

