# Union-Find

Disjoint-set forest over dense integer elements supporting near-constant-time
set merging and membership queries.

## Required API

```rust
pub struct UnionFind { /* fields private */ }

impl UnionFind {
    pub fn new(element_count: usize) -> Self;
    pub fn find(&mut self, element: usize) -> Option<usize>;
    pub fn union(&mut self, a: usize, b: usize) -> Option<bool>;
    pub fn connected(&mut self, a: usize, b: usize) -> Option<bool>;
    pub fn set_count(&self) -> usize;
}
```

The checked-in source is still the scaffold stub `pub fn exercise()`,
which panics via `todo!`; the ignored test marks the unimplemented state.

## Contract

- Elements are dense indexes `0..n`, each initially its own singleton set;
  out-of-range elements yield `None`.
- `find` returns the set representative and applies path compression — note
  it takes `&mut self` because compression mutates parent links during a read.
- `union` links by rank (or size); unioning elements already in the same set
  reports no-op (`Some(false)`) and must not change ranks or the set count.
- `connected(a, b)` is equivalent to comparing `find(a)` and `find(b)`.
- The representative may change across unions; callers may rely only on
  representative equality within a set.
- The set count starts at n and decreases by exactly one per effective union.

## Complexity Targets

- `find`, `union`, `connected`: amortized O(alpha(n)) with path compression
  plus union by rank — effectively constant for all practical n
- Construction: O(n)
- Space: O(n) for parent and rank arrays
