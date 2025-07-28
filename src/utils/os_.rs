use dirs::home_dir;

/// Wrapper around dirs::home_dir which converts PathBuf to a string
pub fn get_home_dir() -> Option<String> {
    match home_dir() {
        Some(path) => Some(path.display().to_string()),
        None => None,
    }
}

