# Resizable Separate-Chaining Hash Table

Generic key-value map that keeps collision chains short by growing and
rehashing its bucket array as entries are added. Keys and values are owned by
the table.

## How It Works

Hash the key to choose a bucket, then store collisions in that bucket's linked
chain. Before an insertion would push the load factor above 0.75, allocate a
bucket array with double the current capacity and rehash every existing entry:
the old bucket index is no longer valid because the calculation uses the new
capacity. Rehashing is expensive once, but capacity doubling makes that cost
amortized across many inserts, preserving O(1) expected operations as the
table grows.

## Required API

```rust
pub struct ResizableHashTable<K, V> { /* fields private */ }

impl<K: core::hash::Hash + Eq, V> ResizableHashTable<K, V> {
    pub fn new() -> Self;
    pub fn insert(&mut self, key: K, value: V) -> Result<Option<V>, (K, V)>;
    pub fn get(&self, key: &K) -> Option<&V>;
    pub fn remove(&mut self, key: &K) -> Option<V>;
    pub fn contains_key(&self, key: &K) -> bool;
    pub fn len(&self) -> usize;
    pub fn capacity(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}

impl<K: core::hash::Hash + Eq, V> Default for ResizableHashTable<K, V> {
    fn default() -> Self;
}
```

## Contract

- Keys require `Hash + Eq`; equal keys must hash equally, and bucket
  membership is decided by `Eq`, never by hash value alone.
- `new` starts with 10 zeroed buckets.
- `insert` adds a new key or replaces the value for an equal key, returning
  `Ok(Some(old_value))` for a replacement and `Ok(None)` for a new key.
  Replacement retains the first stored key.
- `get`/`remove`/`contains_key` take keys by reference; absent keys yield
  `None`/`false` without modifying the table.
- Entries remain correct when distinct keys collide into one bucket; lookups
  traverse the chain comparing with `Eq`.
- Before an insertion would make `len / capacity` exceed 0.75, capacity
  doubles and every entry is rehashed using `hash(key) % new_capacity`.
- If allocating the new bucket array fails, `insert` returns `Err((key, value))`
  and leaves the table and its capacity unchanged.
- The table does not shrink automatically after removals.
- Dropping the table drops all owned keys and values exactly once. Do not
  delegate to `std::collections::HashMap`.

## Complexity Targets

- `insert`, `get`, `remove`, `contains_key`: O(1) expected amortized; O(n)
  worst case
- Resize/rehash: O(n), amortized across inserts
- `len`, `capacity`, `is_empty`: O(1)
- Space: O(bucket capacity + entries)
