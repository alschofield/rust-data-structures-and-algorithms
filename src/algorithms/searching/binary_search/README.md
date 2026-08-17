# Binary Search

Divide-and-conquer search over a sorted slice that halves the candidate range
on every comparison.

## Required API

```rust
pub fn exercise();
```

The module exposes only this scaffold entry point, which panics via `todo!`
when invoked. The contract below specifies a search of the shape `fn
binary_search<T: Ord>(items: &[T], target: &T) -> Option<usize>`.

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

## Learning Focus

Binary search is short but notoriously easy to get subtly wrong: off-by-one
bounds, non-terminating loops, and midpoint overflow are classic defects.
Implementing it builds the loop-invariant discipline ("the target, if present,
is always inside the current range") that generalizes to partition-based
algorithms, and `usize` arithmetic makes the overflow and empty-range edge
cases pleasantly unforgiving.

Status: scaffold — the source is a `todo!` stub; the ignored test marks the
unimplemented state.
