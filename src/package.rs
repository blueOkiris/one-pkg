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

#[derive(Deserialize, Debug, Clone, Copy)]
enum Format {
    Dnf,
    Apt,
    Pacman,
    Aur,
    Flathub,
    AppImage,
    GitHub
}

#[derive(Deserialize, Debug, Clone)]
struct InstallInfo {
    pub name: String,
    pub method: Format
}

impl Package {
    pub fn install(name: &str) -> Result<(), String> {
        println!("Installing package {}.", name);

        // Check if something is already installed
        let installed = Self::installed();
        for pkg in installed {
            if pkg.name == name {
                println!("Package '{}' already installed.", name);
                return Ok(());
            }
        }

        // Find the package with the correct name
        let mut pkg = None;
        let pkgs = Package::load()?;
        for package_desc in pkgs {
            if package_desc.name == name {
                pkg = Some(package_desc.clone())
            }
        }
        if pkg.is_none() {
            return Err(String::from("Package '{}' has no install candidate."));
        }

        Ok(())
    }

    pub fn uninstall(name: &str) -> Result<(), String> {
        println!("Uninstalling package {}.", name);
        Ok(())
    }

    fn load() -> Result<Vec<Self>, String> {
        // Make sure the package list exists
        let conf = config_dir();
        if conf.is_none() {
            return Err(String::from("Failed to find a config directory for OS."));
        }
        let mut conf = conf.unwrap();
        conf.push("one-pkg");
        conf.push("pkg-ls.json");
        if !conf.exists() {
            update_repo()
        }

        // Open, read, and parse the file
        let pkg_ls_file = File::open(conf);
        if pkg_ls_file.is_err() {
            return Err(format!(
                "Couldn't open pkg_ls.json. Error: {}.", pkg_ls_file.err().unwrap()
            ));
        }
        let mut pkg_ls_file = pkg_ls_file.unwrap();
        let mut pkg_ls_str = String::new();
        let read_res = pkg_ls_file.read_to_string(&mut pkg_ls_str);
        if read_res.is_err() {
            return Err(format!(
                "Failed to read all text in pkg_ls.json. Error: {}", read_res.err().unwrap()
            ));
        }
        let pkgs_res: Result<Vec<Package>, _> = from_str(pkg_ls_str.as_str());
        match pkgs_res {
            Ok(pkgs) => Ok(pkgs),
            Err(err) => Err(err.to_string())
        }
    }

    fn installed() -> Vec<InstallInfo> {
        let mut conf = config_dir().expect("Failed to fina config directory for OS.");
        conf.push("one-pkg");
        conf.push("pkg-ls.json");
        if !conf.exists() {
            return Vec::new();
        }

        conf.push("installed.json");
        if !conf.exists() {
            return Vec::new();
        }

        let mut installed_file = File::open(conf).expect("Couldn't open installed list file.");
        let mut installed_str = String::new();
        installed_file.read_to_string(&mut installed_str)
            .expect("Failed to read all text in installed.json");
        let insts_res: Result<Vec<InstallInfo>, _> = from_str(installed_str.as_str());
        match insts_res {
            Ok(insts) => insts,
            Err(_) => Vec::new()
        }
    }
}

pub fn update_repo() {
    println!("Updating repo.");

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

pub fn auto_uninstall() {
    println!("Auto-uninstalling packages.");
}

