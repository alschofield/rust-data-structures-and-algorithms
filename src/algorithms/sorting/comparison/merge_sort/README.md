# Merge Sort

Divide-and-conquer comparison sort that recursively sorts halves and merges
them with an auxiliary buffer.

## Required API

```rust
pub fn exercise();
```

The module exposes only this scaffold entry point, which panics via `todo!`
when invoked. The contract below specifies a sort of the shape `fn
merge_sort<T: Ord + Clone>(items: &mut [T])` (or an ownership-based variant
avoiding `Clone`).

## Contract

- Sorts into ascending order under `Ord`.
- Stable: on ties the merge step must take from the left run first.
- Guaranteed O(n log n) regardless of input order; no adversarial input
  degrades it.
- Uses an O(n) auxiliary buffer; the merge must handle uneven halves and runs
  that exhaust at different times without indexing out of bounds.
- Empty and single-element slices are no-ops; there is no panic path.
- Do not delegate to `slice::sort` (which is itself a merge sort).

## Complexity Targets

- Best: O(n log n)
- Average: O(n log n)
- Worst: O(n log n)
- Space: O(n) auxiliary buffer (plus O(log n) recursion depth)

## Learning Focus

Merge sort is the canonical divide-and-conquer algorithm and the cleanest
proof that comparison sorting can be O(n log n) worst case. Implementing the
merge step precisely — left-first on ties, correct exhaustion handling — shows
where stability actually comes from. In Rust, deciding between a `Clone`-based
buffer and `split_at_mut` ownership juggling is itself an instructive design
decision.

Status: scaffold — the source is a `todo!` stub; the ignored test marks the
unimplemented state.
