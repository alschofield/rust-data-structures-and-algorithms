# Binary Search

Divide-and-conquer search over a sorted slice that halves the candidate range
on every comparison.

## How It Works

Guided elimination over sorted input. Check the middle element: too small,
and the target can only be right of it; too large, only left. Either way half
the candidates disappear, so the search finishes in O(log n) comparisons. The
invariant that keeps the implementation honest: if the target exists, it is
always inside the current [low, high] window — every step must shrink the
window or exit. The famous defects are boundary bugs: midpoint overflow
(hence `low + (high - low) / 2`), off-by-one window updates, and loops that
stop shrinking.

## Required API

```rust
pub fn binary_search<T: Ord>(items: &[T], target: &T) -> Option<usize>;
```


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
