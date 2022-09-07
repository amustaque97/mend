use crate::lib::service::commands::start;
use crate::lib::service::system;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

#[allow(unused_must_use)]
pub fn service_load(formula: &str) {
    // todo: add condition to check if root and not launch on boot `if-else` block
    // https://github.com/Homebrew/homebrew-services/blob/a8f4e6d6d30386a5fbb70ba271886dcc1f3ca0f7/lib/service/services_cli.rb#L248-L252
    if system::check_if_launchctl_exists().is_file() {
        let file = start::find_formula_plist_file(&formula);
        if file.is_file() {
            let file_name = &file.file_name().unwrap();
            if check_if_formula_already_running(&file) {
                println!("{} is already running...", formula);
                exit(1);
            }
            let new_path = format!("{}/{}", dest(), file_name.to_str().unwrap());
            if !Path::new(&new_path).exists() {
                fs::copy(file, &new_path).expect("Failed to copy plist file");
            }
            launchctl_load(&new_path);
        }
    }
}

pub fn dest() -> String {
    if system::is_root() {
        return system::boot_path().into_os_string().into_string().unwrap();
    } else {
        return system::user_path().into_os_string().into_string().unwrap();
    }
}

pub fn launchctl_load(file: &str) -> Result<(), Box<dyn Error>> {
    let _output = Command::new("launchctl")
        .arg("bootstrap")
        .arg(system::domain_target())
        .arg(file)
        .status();

    Ok(())
}

pub fn check_if_formula_already_running(file: &PathBuf) -> bool {
    let label = file.as_path().file_stem().unwrap();

    let command = Command::new("launchctl")
        .arg("list")
        .arg(label)
        .output()
        .expect("Failed to understand whether service is already running or not");

    let output = String::from_utf8(command.stdout).unwrap();

    // Special syntax to write RAW STRING
    let re = Regex::new(r#""PID" = ([0-9]*);"#).unwrap();

    re.is_match(&output)
}
