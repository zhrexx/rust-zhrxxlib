use std::env;
use std::io::{stdin, stdout, Write};
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};
use std::hint::spin_loop;

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
    let mut s = String::new();
    print!("{}", prompt);
    let _= stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
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
    env::args().collect()
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

pub struct TMutex {
    locked: AtomicBool,
}

impl TMutex {
    pub const fn new() -> Self {
        TMutex {
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        while self.locked.swap(true, Ordering::Acquire) {
            while self.locked.load(Ordering::Relaxed) {
                spin_loop();
            }
        }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }

    pub fn guard(&self) -> TMutexGuard<'_> {
        self.lock();
        TMutexGuard { mutex: self }
    }
}

pub struct TMutexGuard<'a> {
    mutex: &'a TMutex,
}

impl<'a> Drop for TMutexGuard<'a> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}

pub struct SharedMutable<T> {
    value: UnsafeCell<T>,
    mutex: TMutex,
}

impl<T> SharedMutable<T> {
    pub const fn new(value: T) -> Self {
        SharedMutable {
            value: UnsafeCell::new(value),
            mutex: TMutex::new(),
        }
    }

    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        let _guard = self.mutex.guard();
        unsafe { f(&*self.value.get()) }
    }

    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let _guard = self.mutex.guard();
        unsafe { f(&mut *self.value.get()) }
    }

    pub fn get(&self) -> &T {
        unsafe {&*self.value.get()}
    }

    pub fn get_mut(&self) -> &mut T {
        unsafe {&mut *self.value.get()}
    }
}

impl<T: Copy> SharedMutable<T> {
    pub fn get_copy(&self) -> T {
        self.with(|v| *v)
    }

    pub fn set(&self, val: T) {
        self.with_mut(|v| *v = val);
    }
}

unsafe impl<T: Send> Send for SharedMutable<T> {}
unsafe impl<T: Send> Sync for SharedMutable<T> {}

