# Dynamic Array

Generic resizable contiguous collection owning its elements, with indexed
access and shifting insert/remove.

## How It Works

A plain array that lies about its size. It keeps capacity (allocated) apart
from size (used); append writes at index size and increments. When capacity
runs out, allocate roughly double, copy everything once, and continue. The
doubling is why append is O(1) amortized: an occasional O(n) copy is paid
for by the n cheap appends that preceded it. Indexing is pure arithmetic,
which is also why the structure is the cache-friendliest thing here.

## Required API

```rust
pub struct DynamicArray<T> { /* fields private */ }

impl<T> DynamicArray<T> {
    pub fn new() -> Self;
    pub fn insert(&mut self, index: usize, item: T) -> Result<(), T>;
    pub fn get(&self, index: usize) -> Option<&T>;
    pub fn set(&mut self, index: usize, item: T) -> Option<T>;
    pub fn remove(&mut self, index: usize) -> Option<T>;
    pub fn len(&self) -> usize;
    pub fn capacity(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}

impl<T> Default for DynamicArray<T> {
    fn default() -> Self;
}
```

## Contract

- Valid element indexes are `[0, len)`; `insert` also accepts `len` to append.
- `insert` at an out-of-range index fails with `Err(item)`, returning
  ownership of the rejected item to the caller; the array is unchanged.
- `get` returns `None` out of range; `set` returns `None` out of range and
  otherwise replaces and returns the old element. `remove` returns `None` out
  of range and otherwise removes and returns the element, shifting the tail left.
- Capacity grows geometrically; capacity is always at least `len`.
- Elements are owned by the array; dropping the array drops all remaining
  elements exactly once.
- `is_empty()` is equivalent to `len() == 0`, and `Default` matches `new`.
- No trait bounds on `T`; do not delegate to `Vec`.

## Complexity Targets

- `get`, `set`, `len`, `capacity`, `is_empty`: O(1)
- `insert` at `len`: amortized O(1)
- `insert` elsewhere and `remove`: O(n) shift
- Space: O(n) contiguous
