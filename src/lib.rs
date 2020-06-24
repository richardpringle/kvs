use std::{collections::HashMap, hash::Hash};

/// Currently there's no real reason to use `KvStore` over using a regular `HashMap`
pub struct KvStore<K, V> {
    map: HashMap<K, V>,
}

impl<K, V> KvStore<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    /// Creates a new `KvStore`
    /// ```
    /// use KvStore;
    ///
    /// let mut store = KvStore::new();
    /// store.set("hello", "world");
    ///
    /// assert_eq!(store.get("hello"), Some("world"));
    /// ```
    pub fn new() -> Self {
        let map = HashMap::new();
        Self { map }
    }

    pub fn get(&self, key: K) -> Option<V> {
        self.map.get(&key).map(|x| x.to_owned())
    }

    pub fn set(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }

    /// Removes a key-value pair from the `KvStore`
    ///
    /// ```
    /// use KvStore;
    ///
    /// let mut store = KvStore::new();
    /// store.set("hello", "world");
    /// assert_eq!(store.get("hello"), Some("world"));
    ///
    /// store.remove("hello");
    ///
    /// assert_eq!(store.get("hello"), None);
    /// ```
    pub fn remove(&mut self, key: K) {
        self.map.remove(&key);
    }
}
