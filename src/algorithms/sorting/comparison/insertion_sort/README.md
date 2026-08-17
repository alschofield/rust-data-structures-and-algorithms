# Insertion Sort

Comparison sort that grows a sorted prefix by shifting each new element left
until it reaches its correct position.

## Required API

```rust
pub fn exercise();
```

The module exposes only this scaffold entry point, which panics via `todo!`
when invoked. The contract below specifies a sort of the shape `fn
insertion_sort<T: Ord>(items: &mut [T])`.

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

## Learning Focus

Insertion sort is the standard example of an adaptive algorithm: its cost is
proportional to the number of inversions, not just n, which is why it serves
as the small-subarray base case inside merge and quick sort. Implementing the
shift-then-place pattern in safe Rust (rotate or swap-walk rather than holding
an element out of a borrowed slice) is a useful exercise in expressing a C
idiom under ownership rules.

Status: scaffold — the source is a `todo!` stub; the ignored test marks the
unimplemented state.
