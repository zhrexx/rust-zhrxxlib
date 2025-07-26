use std::any::TypeId;
use std::any::Any;
use ZHRXXLib::*;

struct A;

fn main() {
    let a = A;
    println!("{}", is!(a, A));
}