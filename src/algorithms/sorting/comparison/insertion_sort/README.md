# Insertion Sort

Comparison sort that grows a sorted prefix by shifting each new element left
until it reaches its correct position.

## Required API

```rust
pub fn insertion_sort<T: Ord>(items: &mut [T]);
```

The checked-in source is still the scaffold stub `pub fn exercise()`,
which panics via `todo!`; the ignored test marks the unimplemented state.

## Contract

- Sorts the slice in place into ascending order under `Ord`.
- Stable: shift while strictly greater, insert after equal elements, so equal
  elements keep their original relative order.
- Adaptive: nearly sorted input costs close to O(n); each element moves only
  as far as its displacement.
- Empty and single-element slices are no-ops; there is no panic path.
- Invariant: before processing index `i`, the range `[0, i)` is sorted.
- Do not delegate to `slice::sort` or `slice::sort_unstable`.

## Complexity Targets

- Best: O(n) (already sorted input, one comparison per element)
- Average: O(n^2)
- Worst: O(n^2) (reverse-sorted input)
- Space: O(1), in place
