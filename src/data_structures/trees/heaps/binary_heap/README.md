# Binary Heap

Array-backed complete binary tree maintaining the heap property, implemented
without `std::collections::BinaryHeap`.

## Required API

```rust
pub fn new<T>() -> T;
```

The module exposes only this scaffold entry point, which panics via `todo!`
when invoked. The contract below specifies a `BinaryHeap<T: Ord>` type
offering `new`, `push`, `pop -> Option<T>`, `peek -> Option<&T>`, `len`, and
`is_empty`.

## Contract

- Implicit array layout: children of index `i` at `2i + 1` and `2i + 2`,
  parent at `(i - 1) / 2`; the occupied prefix is always a complete tree.
- Heap property holds after every operation: each parent orders at or before
  its children under `Ord`.
- `push` appends then sifts up; `pop` swaps root with last, shrinks, then
  sifts down, returning the extreme element by value.
- `pop` and `peek` return `None` on an empty heap; neither panics.
- Equal-priority elements pop in no guaranteed order; the heap is not stable.
- Elements are owned by the heap; dropping it drops all remaining elements
  exactly once.

## Complexity Targets

- `push`: O(log n) (amortized, including geometric growth)
- `pop`: O(log n)
- `peek`, `len`, `is_empty`: O(1)
- Build from n items via bottom-up heapify: O(n)
- Space: O(n) contiguous, no per-element pointer overhead

## Learning Focus

The heap demonstrates that a complete tree needs no pointers — index
arithmetic is the structure. Implementing sift-up and sift-down as dual
restoration passes, and understanding why bottom-up construction is O(n)
while n pushes cost O(n log n), builds the amortized-analysis instincts used
by heap sort, Dijkstra, and A-star downstream in this curriculum.

Status: scaffold — the source is a `todo!` stub; the ignored test marks the
unimplemented state.
