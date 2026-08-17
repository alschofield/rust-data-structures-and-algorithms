/// Separate-chaining key-value map scaffold.
pub struct HashTable<K, V> {
    _marker: core::marker::PhantomData<(K, V)>,
}

impl<K: core::hash::Hash + Eq, V> HashTable<K, V> {
    pub fn new() -> Self {
        todo!("implement HashTable::new")
    }
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let _ = (key, value);
        todo!("implement insert")
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        let _ = key;
        todo!("implement get")
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let _ = key;
        todo!("implement remove")
    }
    pub fn contains_key(&self, key: &K) -> bool {
        let _ = key;
        todo!("implement contains_key")
    }
    pub fn len(&self) -> usize {
        todo!("implement len")
    }
    pub fn is_empty(&self) -> bool {
        todo!("implement is_empty")
    }
}

impl<K: core::hash::Hash + Eq, V> Default for HashTable<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
