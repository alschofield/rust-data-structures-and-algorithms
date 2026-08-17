# Quick Sort

In-place divide-and-conquer comparison sort that partitions around a pivot and
recursively sorts both sides.

## Required API

```rust
pub fn exercise();
```

The module exposes only this scaffold entry point, which panics via `todo!`
when invoked. The contract below specifies a sort of the shape `fn
quick_sort<T: Ord>(items: &mut [T])`.

## Contract

- Sorts the slice in place into ascending order under `Ord`.
- Not stable; partitioning moves equal elements across each other. Do not
  claim stability.
- Pivot selection must defend against sorted and reverse-sorted input
  (median-of-three or randomized), not fixed first/last element.
- Partition invariant: after partitioning, the pivot is in its final position
  with all smaller elements left of it and all larger elements right of it.
- Must remain correct on all-equal input and slices full of duplicates
  without degrading to unbounded recursion.
- Empty and single-element slices are no-ops; there is no panic path.
- Do not delegate to `slice::sort_unstable` (which is a pattern-defeating
  quicksort).

## Complexity Targets

- Best: O(n log n)
- Average: O(n log n)
- Worst: O(n^2) (adversarial pivots)
- Space: O(log n) expected recursion depth (recurse on the smaller side),
  in place otherwise

## Learning Focus

Quick sort teaches partitioning, the workhorse primitive behind quickselect
and many divide-and-conquer routines, and is the clearest case study in
average-case versus worst-case analysis. In Rust, recursing on
`split_at_mut` halves expresses the disjointness of the two partitions in the
type system — the borrow checker enforces what a C programmer must argue by
hand.

Status: scaffold — the source is a `todo!` stub; the ignored test marks the
unimplemented state.
