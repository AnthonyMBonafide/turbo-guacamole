use std::sync::Arc;

use dashmap::DashMap;

pub struct Cache<T>(Arc<DashMap<String, T>>);

impl<T> Cache<T> {
    pub fn new() -> Self {
        Cache(Arc::new(DashMap::<String, T>::new()))
    }
}

impl<T> Default for Cache<T> {
    fn default() -> Self {
        Self::new()
    }
}
