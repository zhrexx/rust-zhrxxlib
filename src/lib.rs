use std::alloc::System;
use std::env;
use std::io::{stdin, stdout, Write};
use utils::logger::*;
use std::any::{Any, TypeId};

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

#[macro_export]
macro_rules! is {
    ($val:expr, $ty:ty) => {{
        fn check<T: Any + 'static>(val: &T) -> bool {
            val.type_id() == TypeId::of::<$ty>()
        }
        check(&$val)
    }};
}

#[macro_export]
macro_rules! implements {
    ($val:expr, $trait:path) => {{
        use std::any::Any;
        ($val as &dyn Any).downcast_ref::<&dyn $trait>().is_some()
    }};
}




