# Heap Sort

Comparison sort that builds a max-heap in the slice, then repeatedly swaps the
root to the tail and re-heapifies the shrinking prefix.

## Required API

```rust
pub fn heap_sort<T: Ord>(items: &mut [T]);
```

The checked-in source is still the scaffold stub `pub fn exercise()`,
which panics via `todo!`; the ignored test marks the unimplemented state.

## Contract

- Sorts the slice in place into ascending order under `Ord`.
- Not stable; sift operations reorder equal elements. Do not claim stability.
- Heap construction must use bottom-up heapify (sift-down from the last
  parent), which is O(n), not n repeated insertions.
- Uses implicit indexing: children of `i` at `2i + 1` and `2i + 2`; no
  allocation.
- Guaranteed O(n log n) regardless of input order.
- Empty and single-element slices are no-ops; there is no panic path.
- Do not delegate to `slice::sort`, `slice::sort_unstable`, or
  `std::collections::BinaryHeap`.

## Complexity Targets

- Best: O(n log n)
- Average: O(n log n)
- Worst: O(n log n)
- Space: O(1), in place and iterative
