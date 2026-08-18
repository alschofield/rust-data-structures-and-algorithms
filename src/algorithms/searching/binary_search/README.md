# Binary Search

Divide-and-conquer search over a sorted slice that halves the candidate range
on every comparison.

## Required API

```rust
pub fn binary_search<T: Ord>(items: &[T], target: &T) -> Option<usize>;
```

The checked-in source is still the scaffold stub `pub fn exercise()`,
which panics via `todo!`; the ignored test marks the unimplemented state.

## Contract

- Input must already be sorted ascending under `Ord`; the function may assume
  but never verify or re-sort it.
- Midpoint arithmetic must not overflow (`low + (high - low) / 2`).
- Returns `Some(index)` of a matching element, `None` when absent; when
  duplicates exist any matching index is acceptable unless a lower-bound
  variant is specified.
- An empty slice returns `None`; there is no panic path.
- The input slice is borrowed immutably and never modified.
- Do not delegate to `slice::binary_search`.

## Complexity Targets

- Best: O(1) (target at first midpoint)
- Average: O(log n)
- Worst: O(log n)
- Space: O(1) iterative
