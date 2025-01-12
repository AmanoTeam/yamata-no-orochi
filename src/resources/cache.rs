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
    map: Arc<RwLock<HashMap<K, V>>>,
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
            map: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Inserts a value into the cache.
    pub async fn insert(&self, key: K, value: V) {
        let mut map = self.map.write().await;
        map.insert(key, value);
    }

    /// Retrieves a value from the cache.
    pub fn get(&self, key: &K) -> Option<V> {
        let map = self.map.try_read().expect("Failed to lock the cache.");
        map.get(key).cloned()
    }

    /// Removes a value from the cache.
    pub async fn remove(&self, key: &K) {
        let mut map = self.map.write().await;
        map.remove(key);
    }
}
