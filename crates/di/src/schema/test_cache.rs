//! Cache trait definitions and implementations for DI container tests.
#![allow(dead_code)]
use crate::prelude::*;

/// Retrieve a value from the cache by key.
pub trait Get: Send + Sync {
    /// Look up a cached value.
    fn get(&self, key: &str) -> Option<String>;
}

/// Store a value in the cache by key.
pub trait Set: Send + Sync {
    /// Insert a value into the cache.
    fn set(&self, key: &str, value: &str);
}

/// In-memory cache implementing [`Get`] and [`Set`].
pub struct MemoryCache {
    /// Cached entries.
    entries: Mutex<HashMap<String, String>>,
}

impl Get for MemoryCache {
    fn get(&self, key: &str) -> Option<String> {
        let entries = self.entries.lock().expect("should be able to lock entries");
        entries.get(key).cloned()
    }
}

impl Set for MemoryCache {
    fn set(&self, key: &str, value: &str) {
        let mut entries = self.entries.lock().expect("should be able to lock entries");
        entries.insert(String::from(key), String::from(value));
    }
}

impl FromProvider for MemoryCache {
    type Error = ResolveError;

    fn from_provider(_services: &ServiceProvider) -> Result<Self, Report<ResolveError>> {
        Ok(Self {
            entries: Mutex::new(HashMap::new()),
        })
    }
}

#[cfg(feature = "async")]
/// Async cache implementing [`Get`] and [`Set`].
pub struct AsyncCache {
    /// Cached entries.
    entries: Mutex<HashMap<String, String>>,
}

#[cfg(feature = "async")]
impl Get for AsyncCache {
    fn get(&self, key: &str) -> Option<String> {
        let entries = self.entries.lock().expect("should be able to lock entries");
        entries.get(key).cloned()
    }
}

#[cfg(feature = "async")]
impl Set for AsyncCache {
    fn set(&self, key: &str, value: &str) {
        let mut entries = self.entries.lock().expect("should be able to lock entries");
        entries.insert(String::from(key), String::from(value));
    }
}

#[cfg(feature = "async")]
impl FromProviderAsync for AsyncCache {
    type Error = ResolveError;

    async fn from_provider_async(
        _services: &ServiceProvider,
    ) -> Result<Self, Report<ResolveError>> {
        Ok(Self {
            entries: Mutex::new(HashMap::new()),
        })
    }
}

/// Mock cache that always returns [`None`].
pub struct MockCache;

impl Get for MockCache {
    fn get(&self, _key: &str) -> Option<String> {
        None
    }
}

impl FromProvider for MockCache {
    type Error = ResolveError;

    fn from_provider(_services: &ServiceProvider) -> Result<Self, Report<ResolveError>> {
        Ok(Self)
    }
}
