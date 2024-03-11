use std::{collections::HashMap, time::SystemTime};
const DEFAULT_CAPACITY: usize = 10;
const DEFAULT_TTL: u128 = 0;

#[derive(Debug, Clone)]
struct Cache<T> {
    data: HashMap<String, Entry<T>>,
}

impl<T> Cache<T> {
    pub fn new() -> Cache<T> {
        Self::with_capacity(DEFAULT_CAPACITY)
    }
    pub fn with_capacity(capacity: usize) -> Cache<T> {
        Cache {
            data: HashMap::with_capacity(capacity),
        }
    }

    pub fn get(&self, key: String) -> Option<&T> {
        match self.data.get(&key) {
            Some(e) => e.get_value(),
            None => None,
        }
    }

    pub fn remove(&mut self, key: String) -> Result<Option<T>, String> {
        match self.data.remove(&key) {
            Some(kv) => {
                if !kv.is_expired() {
                    return Ok(Some(kv.value));
                }

                Ok(None)
            }
            None => Ok(None),
        }
    }

    fn put(&mut self, key: String, value: T) -> Result<(), String> {
        self.put_with_ttl(key, value, DEFAULT_TTL)
    }
    fn put_with_ttl(&mut self, key: String, value: T, ttl: u128) -> Result<(), String> {
        self.data.insert(key, Entry { value, ttl });

        Ok(())
    }

    fn clear(&mut self) -> Result<(), String> {
        self.data.clear();

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Entry<T> {
    value: T,
    ttl: u128,
}

impl<T> Entry<T> {
    fn is_expired(&self) -> bool {
        if self.ttl == 0 {
            return false;
        }

        let now_duration = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("valid time");
        if now_duration.as_millis() >= self.ttl {
            return false;
        }

        true
    }

    fn get_value(&self) -> Option<&T> {
        if !self.is_expired() {
            return Some(&self.value);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_size_count() {
        let mut c = Cache::new();
        assert_eq!(c.data.len(), 0);

        let _ = c.put("one".to_string(), "1");
        assert_eq!(c.data.len(), 1);

        let _ = c.put("one".to_string(), "uno");
        assert_eq!(c.data.len(), 1);

        let _ = c.put("two".to_string(), "2");
        assert_eq!(c.data.len(), 2);

        let _ = c.remove("unknown".to_string());
        assert_eq!(c.data.len(), 2);

        let _ = c.remove("one".to_string());
        assert_eq!(c.data.len(), 1);
    }
    #[test]
    fn test_cache_insert_and_get() {
        let mut c = Cache::new();
        assert!(c.put("cat".to_string(), "meow").is_ok());
        assert!(c.put("dog".to_string(), "bark").is_ok());
        assert!(c.put("bird".to_string(), "tweet").is_ok());

        assert_eq!(c.get("cat".to_string()), Some(&"meow"));
        assert_eq!(c.get("dog".to_string()), Some(&"bark"));
        assert_eq!(c.get("bird".to_string()), Some(&"tweet"));
    }
}
