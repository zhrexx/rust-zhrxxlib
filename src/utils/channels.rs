use std::collections::VecDeque;
use std::sync::{Arc, Mutex, Condvar};

pub struct Channel<T: Send> {
    inner: Arc<Inner<T>>,
}

struct Inner<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
}

impl<T: Send> Clone for Channel<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T: Send> Channel<T> {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner {
                queue: Mutex::new(VecDeque::new()),
                available: Condvar::new(),
            }),
        }
    }

    pub fn send(&self, t: T) {
        let mut queue = self.inner.queue.lock().unwrap();
        queue.push_back(t);
        self.inner.available.notify_one();
    }

    pub fn try_recv(&self) -> Option<T> {
        let mut queue = self.inner.queue.lock().unwrap();
        queue.pop_front()
    }

    pub fn recv(&self) -> T {
        let mut queue = self.inner.queue.lock().unwrap();
        loop {
            if let Some(value) = queue.pop_front() {
                return value;
            }
            queue = self.inner.available.wait(queue).unwrap();
        }
    }
}
