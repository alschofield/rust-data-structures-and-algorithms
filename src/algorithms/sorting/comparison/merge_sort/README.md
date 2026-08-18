# Merge Sort

Divide-and-conquer comparison sort that recursively sorts halves and merges
them with an auxiliary buffer.

## Required API

```rust
pub fn merge_sort<T: Ord + Clone>(items: &mut [T]);
```

The checked-in source is still the scaffold stub `pub fn exercise()`,
which panics via `todo!`; the ignored test marks the unimplemented state.
An ownership-based variant avoiding `Clone` is equally acceptable.

## Contract

- Sorts into ascending order under `Ord`.
- Stable: on ties the merge step must take from the left run first.
- Guaranteed O(n log n) regardless of input order; no adversarial input
  degrades it.
- Uses an O(n) auxiliary buffer; the merge must handle uneven halves and runs
  that exhaust at different times without indexing out of bounds.
- Empty and single-element slices are no-ops; there is no panic path.
- Do not delegate to `slice::sort` (which is itself a merge sort).

## Complexity Targets

- Best: O(n log n)
- Average: O(n log n)
- Worst: O(n log n)
- Space: O(n) auxiliary buffer (plus O(log n) recursion depth)
