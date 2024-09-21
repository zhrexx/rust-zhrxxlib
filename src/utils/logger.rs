use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

#[derive(Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

fn get_color(level: &LogLevel) -> String {
    match level {
        LogLevel::Debug => "\x1b[34m",   // Blue
        LogLevel::Info => "\x1b[32m",    // Green
        LogLevel::Warning => "\x1b[33m", // Yellow
        LogLevel::Error => "\x1b[31m",   // Red
    }
        .to_string()
}

fn reset_color() -> String {
    "\x1b[0m".to_string()
}

pub fn log(level: LogLevel, msg: &str, log_to_file: bool) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let color = get_color(&level);
    let reset = reset_color();
    let level_str = format!("{:?}", level).to_uppercase();

    let log_message = format!("{}[{}] {}{}{}", color, level_str, msg, reset, "\n");

    print!("{}", log_message);

    if log_to_file {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("log.txt")
            .expect("Unable to open log file");

        let file_message = format!("{} [{}]: {}{}", timestamp, level_str, msg, "\n");
        file.write_all(file_message.as_bytes()).expect("Unable to write to log file");
    }
}

pub fn debug(msg: &str, log_to_file: bool) {
    log(LogLevel::Debug, msg, log_to_file);
}

pub fn info(msg: &str, log_to_file: bool) {
    log(LogLevel::Info, msg, log_to_file);
}

pub fn warning(msg: &str, log_to_file: bool) {
    log(LogLevel::Warning, msg, log_to_file);
}

pub fn error(msg: &str, log_to_file: bool) {
    log(LogLevel::Error, msg, log_to_file);
}
