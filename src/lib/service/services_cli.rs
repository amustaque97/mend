use crate::lib::service::commands::start;
use crate::lib::service::system;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

#[allow(unused_must_use)]
pub fn service_load(formula: &str) {
    // todo: add condition to check if root and not launch on boot `if-else` block
    // https://github.com/Homebrew/homebrew-services/blob/a8f4e6d6d30386a5fbb70ba271886dcc1f3ca0f7/lib/service/services_cli.rb#L248-L252
    if system::check_if_launchctl_exists().is_file() {
        let file = start::find_formula_plist_file(&formula);
        if file.is_file() {
            let new_path = format!("{}/{}", dest(), file.file_name().unwrap().to_str().unwrap());
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
