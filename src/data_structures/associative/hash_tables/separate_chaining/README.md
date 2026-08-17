# Hash Table (Separate Chaining)

Generic key-value map resolving collisions with per-bucket chains.

## Required API

```rust
pub struct HashTable<K, V> { /* fields private */ }

impl<K: core::hash::Hash + Eq, V> HashTable<K, V> {
    pub fn new() -> Self;
    pub fn insert(&mut self, key: K, value: V) -> Option<V>;
    pub fn get(&self, key: &K) -> Option<&V>;
    pub fn remove(&mut self, key: &K) -> Option<V>;
    pub fn contains_key(&self, key: &K) -> bool;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}

impl<K: core::hash::Hash + Eq, V> Default for HashTable<K, V> {
    fn default() -> Self;
}
```

## Contract

- Keys require `Hash + Eq`; equal keys must hash equally, and bucket
  membership is decided by `Eq`, never by hash value alone.
- `insert` adds a new key or replaces the value for an existing equal key,
  returning the previous value as `Some(old)` and `None` for a new key.
  Replacement keeps the first stored key.
- `get`/`remove`/`contains_key` take keys by reference; absent keys yield
  `None`/`false` without modifying the table.
- Entries remain correct when distinct keys collide into one bucket; lookups
  traverse the chain comparing with `Eq`.
- The table maintains a bounded load factor by growing the bucket array and
  rehashing every entry; `len` counts entries, not buckets.
- Dropping the table drops all owned keys and values exactly once. Do not
  delegate to `std::collections::HashMap`.

## Complexity Targets

- `insert`, `get`, `remove`, `contains_key`: expected O(1) at bounded load
  factor; O(n) worst case when all keys collide
- Rehash: O(n), amortized into inserts
- `len`, `is_empty`: O(1)
- Space: O(n + buckets)

## Learning Focus

Separate chaining shows how expected O(1) is engineered rather than free: a
hash spreads keys, `Eq` confirms identity, and the load factor bounds chain
length. Implementing rehashing teaches why bucket position is derived state
that must be recomputed on growth, and the `insert -> Option<V>` shape mirrors
idiomatic map APIs where replacement hands back the displaced value.

Status: scaffold — the source is a `todo!` stub; the ignored test marks the
unimplemented state.
