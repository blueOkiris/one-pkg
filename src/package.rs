/*
 * Author: Dylan Turner
 * Description: Define a serializable package object and the functions for manipulating it
 */

use std::{
    fs::{
        create_dir_all,
        rename,
        File
    }, process::Command, io::Read
};
use dirs::config_dir;
use serde::Deserialize;
use serde_json::from_str;

// TODO: Update to main
const PKG_LS_URL: &'static str =
    "https://raw.githubusercontent.com/blueOkiris/one-pkg/blueOkiris/install/pkg-ls.json";

#[derive(Deserialize, Debug, Clone)]
pub struct Package {
    pub name: String,
    pub install: FormatInfo
}

#[derive(Deserialize, Debug, Clone)]
pub struct FormatInfo {
    pub dnf: String,
    pub apt: String,
    pub pacman: String,
    pub aur: String,
    pub flathub: String,
    pub appimage: AppImageInfo,
    pub github: GitHubInfo 
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppImageInfo {
    pub link: String,
    pub name: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct GitHubInfo {
    pub repo: String,
    pub steps: Vec<String>,
    pub deps: Vec<String>
}

impl Package {
    pub fn load() -> Result<Vec<Self>, String> {
        // Make sure the package list exists
        let mut conf = config_dir().expect("Failed to find a config directory for OS.");
        conf.push("one-pkg");
        conf.push("pkg-ls.json");
        if !conf.exists() {
            update_repo()
        }

        let mut pkg_ls_file = File::open(conf).expect("Couldn't open pkg_ls.json.");
        let mut pkg_ls_str = String::new();
        pkg_ls_file.read_to_string(&mut pkg_ls_str)
            .expect("Failed to read all text in pkg_ls.json");
        let pkgs_res: Result<Vec<Package>, _> = from_str(pkg_ls_str.as_str());
        match pkgs_res {
            Ok(pkgs) => Ok(pkgs),
            Err(err) => Err(err.to_string())
        }
    }
}

pub fn update_repo() {
    // Create .config directory
    let mut conf = config_dir().expect("Failed to find a config directory for OS.");
    conf.push("one-pkg");
    if !conf.exists() {
        println!("One-Pkg config folder does not exist.\nCreating .config/one-pkg/");
        create_dir_all(conf.clone()).expect("Failed to create .config/one-pkg");
    }

    // Make a backup
    let mut repo = conf.clone();
    repo.push("pkg-ls.json");
    let mut old = conf.clone();
    old.push("pkg-ls.json.old");
    if repo.exists() {
        println!("Backing up package list.");
        rename(repo, old).expect("Failed to make backup of package list!");
    }

    // Download new list
    println!("Downloading new pkg-ls.json via curl from '{}'", PKG_LS_URL);
    Command::new("curl")
        .args([
            "-o",
            format!("{}/pkg-ls.json", conf.into_os_string().to_str().unwrap()).as_str(),
            PKG_LS_URL
        ]).output()
        .expect("Failed to download new pkg-ls.json");

    println!("Verifying new repo...");
    match Package::load() {
        Ok(_) => println!("Successfully verified."),
        Err(msg) => println!("Error! Invalid pkg-ls.json: {}\nUpdate failed!", msg)
    }
}

