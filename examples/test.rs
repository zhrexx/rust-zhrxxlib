use std::any::TypeId;
use std::any::Any;
use ZHRXXLib::*;
use ZHRXXLib::utils::windowutil::get_windows;

struct A;

fn main() {
    let a = A;
    println!("{}", is!(a, A));
    let windows = get_windows().unwrap();
    println!("{:?}", windows);
}