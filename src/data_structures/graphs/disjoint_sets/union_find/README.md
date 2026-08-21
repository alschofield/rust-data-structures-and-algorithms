# Union-Find

Disjoint-set forest over dense integer elements supporting near-constant-time
set merging and membership queries.

## How It Works

Answers one question fast while groups keep merging: are these two elements
in the same group? Every element points at a parent; following parents ends
at the group's representative, and two elements share a group exactly when
they reach the same representative. Merging groups means pointing one
representative at the other.

Left alone, parent chains grow long, so two cheap tricks keep the forest
near-flat. Path compression: after walking up to find the root, re-point
every visited node directly at it — the lookup flattens the tree behind
itself (which is why find takes a mutable structure). Union by rank: attach
the shorter tree under the taller root so chains grow only when unavoidable.
Together they make operations effectively constant time.

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
