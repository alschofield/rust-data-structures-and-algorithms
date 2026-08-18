# Bubble Sort

Comparison sort that repeatedly sweeps the slice, swapping adjacent
out-of-order pairs until a full pass makes no swaps.

## Required API

```rust
pub fn bubble_sort<T: Ord>(items: &mut [T]);
```

The checked-in source is still the scaffold stub `pub fn exercise()`,
which panics via `todo!`; the ignored test marks the unimplemented state.

## Contract

- Sorts the slice in place into ascending order under `Ord`.
- Stable: equal elements keep their original relative order (adjacent swaps
  only, never swap on equality).
- Must implement the early-exit optimization: a pass with zero swaps
  terminates the sort.
- Empty and single-element slices are no-ops; there is no panic path.
- After each pass `k`, the largest `k` elements occupy their final positions
  at the tail.
- Do not delegate to `slice::sort` or `slice::sort_unstable`.

## Complexity Targets

- Best: O(n) (already sorted input, early exit after one pass)
- Average: O(n^2)
- Worst: O(n^2) (reverse-sorted input)
- Space: O(1), in place
