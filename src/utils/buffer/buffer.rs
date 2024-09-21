use std::any::Any;

pub struct Buffer {
    size: usize,
    items: Vec<Box<dyn Any>>,
}

impl Buffer {
    pub fn new(size: usize) -> Self {
        Buffer {
            size,
            items: Vec::new(),
        }
    }

    pub fn add<T: Any>(&mut self, object: T) -> Result<(), &'static str> {
        if self.items.len() < self.size {
            self.items.push(Box::new(object));
            Ok(())
        } else {
            Err("Buffer is full")
        }
    }

    pub fn get<T: Any>(&self, index: usize) -> Result<&T, &'static str> {
        self.items.get(index)
            .and_then(|item| item.downcast_ref::<T>())
            .ok_or("Type mismatch or index out of bounds")
    }

    pub fn peek<T: Any>(&self, index: usize) -> Result<&T, &'static str> {
        self.items.get(index)
            .and_then(|item| item.downcast_ref::<T>())
            .ok_or("Type mismatch or index out of bounds")
    }

    pub fn remove(&mut self, index: usize) -> Result<Box<dyn Any>, &'static str> {
        if index < self.items.len() {
            Ok(self.items.remove(index))
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn remove_by_type<T: Any>(&mut self) -> Option<Box<dyn Any>> {
        if let Some(index) = self.items.iter().position(|item| item.is::<T>()) {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn swap(&mut self, index1: usize, index2: usize) -> Result<(), &'static str> {
        if index1 < self.items.len() && index2 < self.items.len() {
            self.items.swap(index1, index2);
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_full(&self) -> bool {
        self.items.len() >= self.size
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn contains<T: Any>(&self) -> bool {
        self.items.iter().any(|item| item.is::<T>())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Box<dyn Any>> {
        self.items.iter()
    }

    pub fn items_of_type<T: Any>(&self) -> Vec<&T> {
        self.items.iter()
            .filter_map(|item| item.downcast_ref::<T>())
            .collect()
    }

    pub fn capacity(&self) -> usize {
        self.size
    }
}
