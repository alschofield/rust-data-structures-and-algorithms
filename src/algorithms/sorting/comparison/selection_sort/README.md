# Selection Sort

Comparison sort that grows a sorted prefix by repeatedly selecting the minimum
of the unsorted remainder and swapping it into place.

## Required API

```rust
pub fn exercise();
```

The module exposes only this scaffold entry point, which panics via `todo!`
when invoked. The contract below specifies a sort of the shape `fn
selection_sort<T: Ord>(items: &mut [T])`.

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

## Learning Focus

Selection sort separates two costs that other sorts blend: comparisons versus
data movement. Its comparison count is fixed regardless of input order, which
demonstrates that "best case" depends on what an algorithm can actually skip.
Implementing it also shows concretely why long-distance swaps destroy
stability, a property adjacent-swap sorts get for free.

Status: scaffold — the source is a `todo!` stub; the ignored test marks the
unimplemented state.
