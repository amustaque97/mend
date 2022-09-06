use std::path::PathBuf;
use users::User;
use users::{get_current_uid, get_current_username, get_user_by_uid};
use which::which;

pub fn check_if_launchctl_exists() -> PathBuf {
    which("launchctl").unwrap()
}

pub fn get_current_user_uid() -> u64 {
    let user: User = get_user_by_uid(get_current_uid()).unwrap();
    user.uid() as u64
}

pub fn is_root() -> bool {
    get_current_user_uid() == 0
}

pub fn boot_path() -> PathBuf {
    PathBuf::from("/Library/LaunchDaemons")
}

pub fn user_path() -> PathBuf {
    let user = get_current_username().unwrap();
    let username: String = user.into_string().unwrap();
    let path: PathBuf = ["/", "Users", &username, "Library", "LaunchAgents"]
        .into_iter()
        .collect();
    path
}

pub fn domain_target() -> String {
    if is_root() {
        return String::from("system");
    }

    return format!("gui/{}", get_current_user_uid());
}
