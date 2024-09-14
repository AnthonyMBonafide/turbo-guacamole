use crate::cache::Cache;

struct CacheManager<T> {
    data: Cache<T>,
}

impl<T> CacheManager<T> {
    fn start(self) {
        // Run backgournd process which will use the TX and RX from a Connection to update Cache
        // as well as sending any updates from the Cache out.
    }

    // expose all cache functions but add code to push out changes.
}
