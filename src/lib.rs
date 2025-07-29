#[cfg(feature = "std")]
use std::io::{stdin, stdout, Write};
#[cfg(feature = "std")]
use std::env;
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};
use core::hint::spin_loop;
use core::ops::Deref;
use spin::Once;
use core::mem::MaybeUninit;

pub mod utils;
pub mod event;

/// Allows to get User Input
///
/// #### Example:
/// ```
/// let input = ZHRXXLib::input("prompt_here");
/// println!("{}" ,input)
/// ```
#[cfg(feature = "std")]
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
#[cfg(feature = "std")]
pub fn get_args() -> Vec<String>{
    env::args().collect()
}


/// Allows to check a function is of type T and is static for references make is!(*reference, T)
#[macro_export]
macro_rules! is {
    ($val:expr, $ty:ty) => {{
        fn check<T: Any + 'static>(val: &T) -> bool {
            val.type_id() == TypeId::of::<$ty>()
        }
        check(&$val)
    }};
}

/// Allows to check if a type implements trait T for references make is!(*reference, T)
#[macro_export]
macro_rules! implements {
    ($val:expr, $trait:path) => {{
        use std::any::Any;
        ($val as &dyn Any).downcast_ref::<&dyn $trait>().is_some()
    }};
}

/// A thread mutex this means it dont holds a value
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

/// Allows to store a mutable value in a static must be used with lazy_static
/// if you not a C Guy like I just use Mutex<T>
/// this implementation is available in no_std
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

/// Simple lazy_static alternative
#[cfg(feature = "lazy")]
pub struct Lazy<T> {
    once: Once,
    value: UnsafeCell<MaybeUninit<T>>,
    initialized: AtomicBool,
    init: fn() -> T,
}

#[cfg(feature = "lazy")]
unsafe impl<T: Sync + Send> Sync for Lazy<T> {}
#[cfg(feature = "lazy")]
unsafe impl<T: Send> Send for Lazy<T> {}

#[cfg(feature = "lazy")]
impl<T> Lazy<T> {
    pub const fn new(init: fn() -> T) -> Self {
        Self {
            once: Once::new(),
            value: UnsafeCell::new(MaybeUninit::uninit()),
            initialized: AtomicBool::new(false),
            init,
        }
    }

    fn get(&self) -> &T {
        self.once.call_once(|| {
            let val = (self.init)();
            unsafe {
                (*self.value.get()).write(val);
            }
            self.initialized.store(true, Ordering::Release);
        });

        assert!(self.initialized.load(Ordering::Acquire));
        unsafe { &*(*self.value.get()).as_ptr() }
    }


}

#[cfg(feature = "lazy")]
impl<T> Deref for Lazy<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

#[cfg(feature = "lazy")]
#[macro_export]
macro_rules! lazy_static {
    (static ref $name:ident: $t:ty = $init:expr;) => {
        static $name: $crate::Lazy<$t> = $crate::Lazy::new(|| $init);
    };

    ($(static ref $name:ident: $t:ty = $init:expr;)*) => {
        $(lazy_static!(static ref $name: $t = $init;);)*
    };
}