/// Logger module used for logging with colors
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;
use crossterm::ExecutableCommand;

#[derive(Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

/// Internal function used by helpers
fn log(level: LogLevel, msg: &str, log_to_file: bool) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let level_str = format!("{:?}", level).to_uppercase();

    let log_message = format!("[{}] {}{}", level_str, msg, "\n");

    let mut stdout = std::io::stdout();
    stdout.execute(crossterm::style::SetForegroundColor(match level {
        LogLevel::Debug => crossterm::style::Color::Blue,
        LogLevel::Info => crossterm::style::Color::Green,
        LogLevel::Warning => crossterm::style::Color::Yellow,
        LogLevel::Error => crossterm::style::Color::Red,
    })).unwrap();
    print!("{}", log_message);
    stdout.execute(crossterm::style::ResetColor).unwrap();

    if log_to_file {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("log.txt")
            .expect("Unable to open log file");

        let file_message = format!("{} {}", timestamp, log_message);
        file.write_all(file_message.as_bytes()).expect("Unable to write to log file");
    }
}

/// Used for Debug prints 
/// NOTE / FEATURE: dont prints if compiled in release mode
pub fn debug(msg: &str, log_to_file: bool) {
    #[cfg(debug_assertions)]
    log(LogLevel::Debug, msg, log_to_file);
}


/// Used for Info prints 
pub fn info(msg: &str, log_to_file: bool) {
    log(LogLevel::Info, msg, log_to_file);
}

/// Used for warnings
pub fn warning(msg: &str, log_to_file: bool) {
    log(LogLevel::Warning, msg, log_to_file);
}

/// Used for errors
pub fn error(msg: &str, log_to_file: bool) {
    log(LogLevel::Error, msg, log_to_file);
}
