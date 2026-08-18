# Selection Sort

Comparison sort that grows a sorted prefix by repeatedly selecting the minimum
of the unsorted remainder and swapping it into place.

## Required API

```rust
pub fn selection_sort<T: Ord>(items: &mut [T]);
```

The checked-in source is still the scaffold stub `pub fn exercise()`,
which panics via `todo!`; the ignored test marks the unimplemented state.

## Contract

- Sorts the slice in place into ascending order under `Ord`.
- Performs at most n - 1 swaps total; this is the algorithm's defining
  advantage when writes are expensive.
- Not stable in its classic swap form; do not claim stability.
- Empty and single-element slices are no-ops; there is no panic path.
- Invariant: after iteration `k`, the first `k` elements are the `k` smallest
  in final sorted order.
- Do not delegate to `slice::sort` or `slice::sort_unstable`.

## Complexity Targets

- Best: O(n^2) (comparisons do not shrink on sorted input)
- Average: O(n^2)
- Worst: O(n^2)
- Space: O(1), in place
