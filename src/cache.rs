use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Cache<T> {
    size: u64,
    data: HashMap<String, Entry<T>>,
}

impl<T> Cache<T> {
    pub fn new() -> Cache<T> {
        Cache {
            size: 0,
            data: HashMap::new(),
        }
    }
    pub fn with_capacity(capacity: usize) -> Cache<T> {
        Cache {
            size: 0,
            data: HashMap::with_capacity(capacity),
        }
    }

    pub fn get(&self, key: String) -> Option<&T> {
        todo!()
    }

    pub fn remove(&mut self, key: String) -> Result<(), String> {
        todo!()
    }

    fn put(&mut self, key: String, value: T) -> Result<(), String> {
        todo!()
    }
    fn put_with_ttl(&mut self, key: String, value: T, ttl: u64) -> Result<(), String> {
        todo!()
    }

    fn clear(&mut self) -> Result<(), String> {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct Entry<T> {
    value: T,
    ttl: u64,
}
