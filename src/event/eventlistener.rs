use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub trait EventListener: Send + Sync {
    fn on_event(&self, event_name: &str);
}

pub struct Events {
    event_handlers: HashMap<String, Vec<Arc<dyn Fn() + Send + Sync>>>,
}

impl Events {
    pub fn new() -> Self {
        Events {
            event_handlers: HashMap::new(),
        }
    }

    /// Register a listener for a specific event
    pub fn register_listener<T: EventListener + 'static>(&mut self, event_name: &str, listener: T) {
        let listener = Arc::new(listener); // Wrap listener in Arc
        let event_name_owned = event_name.to_string(); // Clone the event_name to own the string

        // Add the listener to the specified event
        self.event_handlers
            .entry(event_name_owned.clone())
            .or_insert_with(Vec::new)
            .push(Arc::new(move || {
                listener.on_event(&event_name_owned);
            }));
    }

    /// Create a new custom event without any listeners initially
    pub fn create_event(&mut self, event_name: &str) {
        self.event_handlers
            .entry(event_name.to_string())
            .or_insert_with(Vec::new);
    }

    /// Trigger an event, executing all registered handlers
    pub fn trigger_event(&self, event_name: &str) {
        if let Some(handlers) = self.event_handlers.get(event_name) {
            for handler in handlers {
                handler();
            }
        } else {
            println!("Event '{}' does not exist or has no handlers.", event_name);
        }
    }
}

#[macro_export]
macro_rules! event {
    ($name:expr) => {
        fn __event_name() -> &'static str {
            $name
        }
    };
}
