use std::alloc::System;
use std::env;
use std::io::{stdin, stdout, Write};
use utils::logger::*;

pub mod utils;
pub mod event;
pub mod error;

/// Allows to get User Input
///
/// #### Example:
/// ```
/// let input = ZHRXXLib::input("prompt_here");
/// println!("{}" ,input)
/// ```
pub fn input(prompt:&str) -> String {
    let mut s=String::new();
    print!("{}", prompt);
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }

    s
}

/// Allows to get Args
///
/// > 0 Object is the path of executable
///
/// # Example:
/// ```
/// let args = ZHRXXLib::get_args();
/// println!("{:?}" ,args)
/// ```
pub fn get_args() -> Vec<String>{
    let args: Vec<String> = env::args().collect();

    args
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_args() {
        env::set_var("CARGO_BIN_NAME", "test_binary");
        let args = get_args();
        assert!(args.len() >= 1, "Expected at least one argument (binary path).");
    }
}



