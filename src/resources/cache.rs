// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The cache resource.

use std::{collections::HashMap, hash::Hash, sync::Arc};

use tokio::sync::RwLock;

/// Cache module.
#[derive(Clone)]
pub struct Cache<K, V> {
    /// The underlying map storing the cached values.
    map: Arc<RwLock<HashMap<K, V>>>,
    /// The maximum size of the cache.
    max_size: usize,
}

#[allow(dead_code)]
impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Creates a new instance of the cache.
    ///
    /// # Arguments
    ///
    /// * `max_size` - The maximum size of the cache.
    pub fn with(max_size: usize) -> Self {
        Self {
            map: Arc::new(RwLock::new(HashMap::new())),
            max_size,
        }
    }

    /// Retrieves a value from the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The key associated with the value to be retrieved.
    pub fn get(&self, key: &K) -> Option<V> {
        let map = self.map.try_read().expect("Failed to lock the cache.");
        map.get(key).cloned()
    }

    /// Inserts a value into the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The key associated with the value to be inserted.
    /// * `value` - The value to be inserted into the cache.
    pub async fn insert(&self, key: K, value: V) {
        let mut map = self.map.write().await;

        if map.len() >= self.max_size {
            map.clear();
        }

        map.insert(key, value);
    }

    /// Removes a value from the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The key associated with the value to be removed.
    pub async fn remove(&self, key: &K) {
        let mut map = self.map.write().await;
        map.remove(key);
    }
}
