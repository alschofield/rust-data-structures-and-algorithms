# Selection Sort

Comparison sort that grows a sorted prefix by repeatedly selecting the minimum
of the unsorted remainder and swapping it into place.

## How It Works

Grow a sorted prefix by selection: scan the unsorted remainder for its
minimum, swap it into the next prefix slot, repeat. Comparisons never shrink
— sorted input still costs a full scan per position, so every case is O(n^2)
— but the sort performs at most n-1 swaps total, its one real advantage when
writes are expensive. The long-distance swap can carry an element past an
equal one, so the classic form is not stable.

## Required API

```rust
pub fn selection_sort<T: Ord>(items: &mut [T]);
```


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
