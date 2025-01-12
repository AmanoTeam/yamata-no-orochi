// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The cache resource.

use std::{collections::HashMap, future::Future, hash::Hash, sync::Arc};

use async_trait::async_trait;
use tokio::sync::Mutex;

/// Cache module.
#[derive(Clone)]
pub struct Cache<K, V> {
    map: Arc<Mutex<HashMap<K, V>>>,
}

#[allow(dead_code)]
impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Creates a new instance of the cache resource.
    pub fn new() -> Self {
        Self {
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Inserts a value into the cache.
    pub async fn insert(&self, key: K, value: V) {
        let mut map = self.map.lock().await;
        map.insert(key, value);
    }

    /// Retrieves a value from the cache.
    pub async fn get(&self, key: &K) -> Option<V> {
        let map = self.map.lock().await;
        map.get(key).cloned()
    }

    /// Removes a value from the cache.
    pub async fn remove(&self, key: &K) {
        let mut map = self.map.lock().await;
        map.remove(key);
    }
}

/// A trait that defines cacheable behavior.
#[allow(dead_code)]
#[async_trait]
pub trait Cacheable {
    /// The type of the key used to identify cached values.
    type Key;

    /// The type of the value to be cached.
    type Value;

    /// Gets a value from the cache or inserts it if it does not exist.
    ///
    /// # Arguments
    ///
    /// * `key` - The key used to identify the cached value.
    /// * `f` - A function that returns a future resolving to the value to be inserted if the key does not exist.
    async fn get_or_insert_with<F, Fut>(&self, key: Self::Key, f: F) -> Self::Value
    where
        F: FnOnce() -> Fut + Send,
        Fut: Future<Output = Self::Value> + Send;
}

#[async_trait]
impl<K, V> Cacheable for Cache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    type Key = K;
    type Value = V;

    async fn get_or_insert_with<F, Fut>(&self, key: K, f: F) -> V
    where
        F: FnOnce() -> Fut + Send,
        Fut: std::future::Future<Output = V> + Send,
    {
        if let Some(value) = self.get(&key).await {
            return value;
        }

        let value = f().await;
        self.insert(key.clone(), value.clone()).await;
        value
    }
}
