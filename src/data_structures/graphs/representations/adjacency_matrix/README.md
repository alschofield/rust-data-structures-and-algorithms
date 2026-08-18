# Adjacency Matrix

Graph representation storing edge presence (or weight) in a V x V grid,
indexed by vertex pair. The standard choice for dense graphs.

## Required API

```rust
pub struct AdjacencyMatrix { /* fields private */ }

impl AdjacencyMatrix {
    pub fn new(vertex_count: usize, directed: bool) -> Self;
    pub fn add_edge(&mut self, from: usize, to: usize) -> Option<bool>;
    pub fn remove_edge(&mut self, from: usize, to: usize) -> Option<bool>;
    pub fn has_edge(&self, from: usize, to: usize) -> Option<bool>;
    pub fn neighbors(&self, vertex: usize) -> Option<impl Iterator<Item = usize> + '_>;
    pub fn vertex_count(&self) -> usize;
    pub fn edge_count(&self) -> usize;
}
```

The checked-in source is still the scaffold stub `pub fn exercise()`,
which panics via `todo!`; the ignored test marks the unimplemented state.

## Contract

- Vertexes are dense indexes `0..vertex_count`; out-of-range vertexes are
  rejected via `Result`/`Option`.
- Backing storage is one contiguous allocation indexed as
  `u * vertex_count + v`; a fresh graph has every cell cleared.
- `has_edge(u, v)` is a single cell read — the representation's defining
  strength.
- Undirected graphs keep the matrix symmetric: `add_edge`/`remove_edge`
  update `(u, v)` and `(v, u)` together.
- Adding an existing edge and removing an absent edge are clean no-ops with
  documented return values.
- Neighbor iteration scans row u in full, even for low-degree vertexes.

## Complexity Targets

- `add_edge`, `remove_edge`, `has_edge`: O(1)
- Iterate neighbors of u: O(V), regardless of degree
- Full traversal of all edges: O(V^2)
- Space: O(V^2), independent of edge count
