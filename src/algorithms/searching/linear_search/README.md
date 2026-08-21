# Linear Search

Sequential scan that finds a target by comparing every element in order until
a match is found or the slice is exhausted.

## How It Works

The honest baseline: examine elements front to back until one matches or the
input is exhausted. No ordering precondition, no preprocessing — this is the
only search that works on arbitrary unsorted data. The contract wrinkle is
duplicates: the scan direction guarantees the first match is the one
reported.

## Required API

```rust
pub fn linear_search<T: PartialEq>(items: &[T], target: &T) -> Option<usize>;
```


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
