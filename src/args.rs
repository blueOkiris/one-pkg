/*
 * Author: Dylan Turner
 * Description: Handle cli arguments into the package manager
 */

use clap::{
    ArgMatches, command, arg, Command
};

pub fn get_args() -> ArgMatches {
    command!()
        .subcommand(
            Command::new("install")
                .about("Install a package from a variety of formats.")
                .arg(arg!([PACKAGE]))
        ).subcommand(
            Command::new("uninstall")
                .about("Uninstall an installed package.")
                .arg(arg!([PACKAGE]))
        ).subcommand(
            Command::new("auto-uninstall")
                .about("Auto-remove leftovers from certain package formats.")
        ).get_matches()
} 

