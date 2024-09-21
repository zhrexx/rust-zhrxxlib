
use std::env::home_dir;

pub fn get_home_dir() -> Option<String> {
    match home_dir() {
        Some(path) => Some(path.display().to_string()),
        None => None,
    }
}

