/*
 * Author: Dylan Turner
 * Description: Define a serializable package object and the functions for manipulating it
 */

use std::{
    fs::{
        create_dir_all,
        rename,
        File
    }, process::Command,
    io::{
        stdin, Read        
    }
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
    pub uninstall_steps: Vec<String>,
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
        let installed = Self::installed()?;
        for pkg in installed {
            if pkg.name == name {
                println!("Package '{}' already installed via {:?}.", name, pkg.method);
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
        let pkg = pkg.unwrap();

        // Select all install options
        let mut opts = Vec::new();
        if pkg.install.dnf != "" {
            opts.push(Format::Dnf);
        }
        if pkg.install.apt != "" {
            opts.push(Format::Apt);
        }
        if pkg.install.pacman != "" {
            opts.push(Format::Pacman);
        }
        if pkg.install.aur != "" {
            opts.push(Format::Aur);
        }
        if pkg.install.flathub != "" {
            opts.push(Format::Flathub);
        }
        if pkg.install.appimage.link != "" {
            opts.push(Format::AppImage);
        }
        if pkg.install.github.repo != "" {
            opts.push(Format::GitHub);
        }

        // Get user input
        println!("The package '{}' is available in the following formats:", pkg.name);
        for i in 0..opts.len() {
            println!("({}) {:?}", i + 1, opts[i]);
        }
        println!("Please enter a number for which format you'd like to use:");
        let mut choice = String::new();
        let inp = stdin();
        let read_res = inp.read_line(&mut choice);
        if read_res.is_err() {
            return Err(format!(
                "Failed to take user input. Error: {}", read_res.err().unwrap()
            ));
        }
        choice.pop();
        let choice_num = choice.parse::<usize>();
        if choice_num.is_err() {
            return Err(format!(
                "Failed to parse input value. Not a number. Error: {}", choice_num.err().unwrap()
            ));
        }
        let choice_num = choice_num.unwrap() - 1;
        if choice_num >= opts.len() {
            return Err(format!(
                "{} was not an option, you stupid idiot.", choice_num
            ));
        }

        // Install
        match opts[0] {
            Format::Dnf => {
                
            }, Format::Apt => {

            }, Format::Pacman => {

            }, Format::Aur => {

            }, Format::Flathub => {
    
            }, Format::AppImage => {

            }, Format::GitHub => {

            }
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
            update_repo()?;
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

    fn installed() -> Result<Vec<InstallInfo>, String> {
        let conf = config_dir();
        if conf.is_none() {
            return Err(String::from("Failed to find a config directory for OS."));
        }
        let mut conf = conf.unwrap();
        conf.push("one-pkg");
        conf.push("pkg-ls.json");
        if !conf.exists() {
            return Ok(Vec::new());
        }

        conf.push("installed.json");
        if !conf.exists() {
            return Ok(Vec::new());
        }

        let installed_file = File::open(conf);
        if installed_file.is_err() {
            return Err(format!(
                "Couldn't open installed list file. Error: {}", installed_file.err().unwrap()
            ));
        }
        let mut installed_file = installed_file.unwrap();
        let mut installed_str = String::new();
        let read_res = installed_file.read_to_string(&mut installed_str);
        if read_res.is_err() {
            return Err(format!(
                "Failed to read all text in installed.json. Error: {}", read_res.err().unwrap()
            ));
        }
        let insts_res: Result<Vec<InstallInfo>, _> = from_str(installed_str.as_str());
        match insts_res {
            Ok(insts) => Ok(insts),
            Err(err) => Err(format!("Corrupted installed.json! Error parsing: {}", err.to_string()))
        }
    }
}

pub fn update_repo() -> Result<(), String> {
    println!("Updating repo.");

    // Create .config directory
    let conf = config_dir();
    if conf.is_none() {
        return Err(String::from("Failed to find a config directory for OS."));
    }
    let mut conf = conf.unwrap();
    conf.push("one-pkg");
    if !conf.exists() {
        println!("One-Pkg config folder does not exist.\nCreating .config/one-pkg/");
        let create = create_dir_all(conf.clone());
        if create.is_err() {
            return Err(format!(
                "Failed to create .config/one-pkg. Error: {}", create.err().unwrap()
            ));    
        }
    }

    // Make a backup
    let mut repo = conf.clone();
    repo.push("pkg-ls.json");
    let mut old = conf.clone();
    old.push("pkg-ls.json.old");
    if repo.exists() {
        println!("Backing up package list.");
        let ren_res = rename(repo, old);
        if ren_res.is_err() {
            return Err(format!(
                    "Failed to make backup of package list! Error: {}", ren_res.err().unwrap()
            ));
        }
    }

    // Download new list
    println!("Downloading new pkg-ls.json via curl from '{}'", PKG_LS_URL);
    let curl_res = Command::new("curl")
        .args([
            "-o",
            format!("{}/pkg-ls.json", conf.into_os_string().to_str().unwrap()).as_str(),
            PKG_LS_URL
        ]).output();
    if curl_res.is_err() {
        return Err(format!(
            "Failed to download new pkg-ls.json. Error: {}", curl_res.err().unwrap()
        ));
    }

    println!("Verifying new repo...");
    Package::load()?;
    Ok(println!("Successfully verified new repo."))
}

pub fn auto_uninstall() -> Result<(), String> {
    println!("Auto-uninstalling packages.");

    Ok(())
}

