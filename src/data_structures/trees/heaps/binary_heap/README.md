# Binary Heap

Array-backed complete binary tree maintaining the heap property, implemented
without `std::collections::BinaryHeap`.

## Required API

```rust
pub struct BinaryHeap<T> { /* fields private */ }

impl<T: Ord> BinaryHeap<T> {
    pub fn new() -> Self;
    pub fn push(&mut self, item: T);
    pub fn pop(&mut self) -> Option<T>;
    pub fn peek(&self) -> Option<&T>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}

impl<T: Ord> Default for BinaryHeap<T> {
    fn default() -> Self;
}
```

The checked-in source is still the scaffold stub `pub fn new<T>() -> T`,
which panics via `todo!`; the ignored test marks the unimplemented state.

## Contract

- Implicit array layout: children of index `i` at `2i + 1` and `2i + 2`,
  parent at `(i - 1) / 2`; the occupied prefix is always a complete tree.
- Heap property holds after every operation: each parent orders at or before
  its children under `Ord`.
- `push` appends then sifts up; `pop` swaps root with last, shrinks, then
  sifts down, returning the extreme element by value.
- `pop` and `peek` return `None` on an empty heap; neither panics.
- Equal-priority elements pop in no guaranteed order; the heap is not stable.
- Elements are owned by the heap; dropping it drops all remaining elements
  exactly once.

## Complexity Targets

- `push`: O(log n) (amortized, including geometric growth)
- `pop`: O(log n)
- `peek`, `len`, `is_empty`: O(1)
- Build from n items via bottom-up heapify: O(n)
- Space: O(n) contiguous, no per-element pointer overhead
