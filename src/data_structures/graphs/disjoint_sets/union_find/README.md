# Union-Find

Disjoint-set forest over dense integer elements supporting near-constant-time
set merging and membership queries.

## Required API

```rust
pub fn exercise();
```

The module exposes only this scaffold entry point, which panics via `todo!`
when invoked. The contract below specifies a `UnionFind` type offering
construction for n elements, `find(&mut self, x: usize) -> Option<usize>`,
`union(&mut self, a: usize, b: usize) -> Option<bool>`, `connected(&mut self,
a: usize, b: usize) -> Option<bool>`, and a live set count.

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

## Learning Focus

Union-find delivers one of the most striking results in data structures: two
small optimizations turn a potentially linear operation into inverse-Ackermann
amortized time. The Rust angle is that path compression makes `find` a
mutating read (`&mut self`), a direct lesson in how the borrow checker
surfaces hidden mutation that other languages let slide. It is the enabling
primitive for Kruskal's MST and connectivity queries.

Status: scaffold — the source is a `todo!` stub; the ignored test marks the
unimplemented state.
