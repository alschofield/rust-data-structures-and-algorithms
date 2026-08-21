# Heap Sort

Comparison sort that builds a max-heap in the slice, then repeatedly swaps the
root to the tail and re-heapifies the shrinking prefix.

## How It Works

Two ideas glued together. First, the array is treated as an implicit tree:
the element at index i has children at 2i+1 and 2i+2, and the max-heap rule
(every parent >= its children) guarantees the largest element sits at index 0
without the array being sorted. The maintenance move is sift-down: a node
smaller than a child swaps with its larger child and repeats from its new
position until it settles.

Second, harvest: swap the root (the maximum) with the last heap element,
shrink the heap by one so that slot is final, and sift the new root down to
restore the rule. Each round locks one element at the tail — bubble sort's
shape, but finding the max costs O(log n) instead of an O(n) pass. The build
step walks backward from the last parent sifting each node down, which is
O(n) because most nodes are near the bottom and barely move. Long-distance
swaps leap over equal elements, which is why the sort cannot be stable.

## Required API

```rust
pub fn heap_sort<T: Ord>(items: &mut [T]);
```


## Contract

- Sorts the slice in place into ascending order under `Ord`.
- Not stable; sift operations reorder equal elements. Do not claim stability.
- Heap construction must use bottom-up heapify (sift-down from the last
  parent), which is O(n), not n repeated insertions.
- Uses implicit indexing: children of `i` at `2i + 1` and `2i + 2`; no
  allocation.
- Guaranteed O(n log n) regardless of input order.
- Empty and single-element slices are no-ops; there is no panic path.
- Do not delegate to `slice::sort`, `slice::sort_unstable`, or
  `std::collections::BinaryHeap`.

## Complexity Targets

- Best: O(n log n)
- Average: O(n log n)
- Worst: O(n log n)
- Space: O(1), in place and iterative
