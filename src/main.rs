/*
 * Author: Dylan Turner
 * Description: Entry point for one-pkg project
 */

mod args;
mod package;

use crate::{
    args::get_args,
    package::{
        Package,
        update_repo,
        auto_uninstall
    }
};

fn main() {
    let command = get_args();
    let sub_cmd = command.subcommand();
    if sub_cmd.is_some() {
        let (name, args) = sub_cmd.unwrap();
        match name {
            "install" => Package::install(
                args.value_of("PACKAGE").expect("No package name provided!")
            ).unwrap(),
            "uninstall" => Package::uninstall(
                args.value_of("PACKAGE").expect("No package name provided!")
            ).unwrap(),
            "auto-uninstall" => auto_uninstall(),
            "update" => update_repo(),
            _ => {} // Handled by clap
    
        }
    } else {
        println!("No command provided. Don't know how to use? You probably want to add --help.");
    }
}

