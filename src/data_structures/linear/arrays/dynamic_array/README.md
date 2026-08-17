# Dynamic Array

Generic resizable contiguous collection owning its elements, with indexed
access and shifting insert/remove.

## Required API

```rust
pub struct DynamicArray<T> { /* fields private */ }

impl<T> DynamicArray<T> {
    pub fn new() -> Self;
    pub fn insert(&mut self, index: usize, item: T) -> Result<(), T>;
    pub fn get(&self, index: usize) -> Option<&T>;
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T>;
    pub fn remove(&mut self, index: usize) -> Option<T>;
    pub fn len(&self) -> usize;
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
- `get`/`get_mut` return `None` out of range; `remove` returns `None` out of
  range and otherwise removes and returns the element, shifting the tail left.
- Capacity grows geometrically; capacity is always at least `len`.
- Elements are owned by the array; dropping the array drops all remaining
  elements exactly once.
- `is_empty()` is equivalent to `len() == 0`, and `Default` matches `new`.
- No trait bounds on `T`; do not delegate to `Vec`.

## Complexity Targets

- `get`, `get_mut`, `len`, `is_empty`: O(1)
- `insert` at `len`: amortized O(1)
- `insert` elsewhere and `remove`: O(n) shift
- Space: O(n) contiguous

## Learning Focus

This is the vector rebuilt from first principles: geometric growth, element
shifting, and the len/capacity distinction. The Rust-specific lesson is in the
signatures — `Err(item)` on rejected insert gives the item back instead of
dropping it, and `get`/`get_mut` show how shared and exclusive borrows of the
same storage are kept apart by the type system.

Status: scaffold — the source is a `todo!` stub; the ignored test marks the
unimplemented state.
