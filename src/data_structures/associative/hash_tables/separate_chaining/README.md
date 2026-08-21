# Hash Table

Generic key-value map resolving collisions with per-bucket chains. Keys and
values are owned by the table; values may be any type.

## How It Works

Hash the key to choose a bucket, then deal with collisions by letting each
bucket hold a linked chain of entries. Lookup hashes, jumps to the bucket, and
walks the chain comparing keys. This implementation uses 10 fixed buckets, so
the chains grow with n instead of resizing or rehashing; average operation
cost therefore grows as O(n / buckets).

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
- `insert` adds a new key or replaces the value for an equal key, returning
  `Some(old_value)` for a replacement and `None` for a new key. Replacement
  retains the first stored key.
- `get`/`remove`/`contains_key` take keys by reference; absent keys yield
  `None`/`false` without modifying the table.
- Entries remain correct when distinct keys collide into one bucket; lookups
  traverse the chain comparing with `Eq`.
- The table has 10 fixed buckets and does not resize or rehash.
- Dropping the table drops all owned keys and values exactly once. Do not
  delegate to `std::collections::HashMap`.

## Complexity Targets

- `insert`, `get`, `remove`, `contains_key`: expected O(1) with short chains,
  O(n / buckets) as fixed-bucket chains grow, O(n) worst case
- `len`, `is_empty`: O(1)
- Space: O(entries + 10 buckets)
