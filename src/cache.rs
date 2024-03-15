use std::sync::Arc;

use dashmap::DashMap;

pub type Cache<T> = Arc<DashMap<String, T>>;
