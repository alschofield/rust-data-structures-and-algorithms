# Linear Search

Sequential scan that finds a target by comparing every element in order until
a match is found or the slice is exhausted.

## Required API

```rust
pub fn exercise();
```

The module exposes only this scaffold entry point, which panics via `todo!`
when invoked. The contract below specifies a search of the shape `fn
linear_search<T: PartialEq>(items: &[T], target: &T) -> Option<usize>`.

## Contract

- Works on unsorted slices; no ordering precondition may be assumed.
- Returns `Some(index)` of the first matching element when duplicates exist,
  `None` when the target is absent.
- An empty slice returns `None`; there is no panic path.
- The input slice is borrowed immutably and never modified.
- Requires only `PartialEq`; do not use `Iterator::position` or other library
  search helpers for the exercise.

## Complexity Targets

- Best: O(1) (target at index 0)
- Average: O(n)
- Worst: O(n) (target absent or last)
- Space: O(1)

## Learning Focus

Linear search is the baseline every other search is measured against. In Rust
the exercise is also about API shape: `Option<usize>` makes not-found a value
rather than a sentinel, and the `PartialEq` bound is the minimal capability
the algorithm actually needs — a first lesson in choosing the loosest
sufficient trait bound.

Status: scaffold — the source is a `todo!` stub; the ignored test marks the
unimplemented state.
