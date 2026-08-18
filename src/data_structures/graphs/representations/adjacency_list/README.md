# Adjacency List

Graph representation storing, for each vertex, a list of its outgoing edges.
The standard choice for sparse graphs.

## Required API

```rust
pub struct AdjacencyList { /* fields private */ }

impl AdjacencyList {
    pub fn new(vertex_count: usize, directed: bool) -> Self;
    pub fn add_edge(&mut self, from: usize, to: usize) -> Option<()>;
    pub fn has_edge(&self, from: usize, to: usize) -> Option<bool>;
    pub fn neighbors(&self, vertex: usize) -> Option<&[usize]>;
    pub fn vertex_count(&self) -> usize;
    pub fn edge_count(&self) -> usize;
}
```

The checked-in source is still the scaffold stub `pub fn exercise()`,
which panics via `todo!`; the ignored test marks the unimplemented state.

## Contract

- Vertexes are dense indexes `0..vertex_count`; out-of-range vertexes are
  rejected via `Result`/`Option`, never by panicking in release paths.
- `add_edge(u, v)` appends v to u's edge list; the directed/undirected policy
  is fixed at construction, and undirected graphs store both directions
  consistently.
- Duplicate edge policy is explicit: either reject duplicates or document
  multigraph behavior; `has_edge` scans only u's list.
- Neighbor iteration yields exactly u's out-edges, each once, in a
  deterministic order, borrowing the graph immutably.
- Self-loops are permitted unless documented otherwise.
- The graph owns its edge storage; dropping it frees everything exactly once.

## Complexity Targets

- `add_edge`: amortized O(1)
- `has_edge(u, v)`: O(deg(u))
- Iterate neighbors of u: O(deg(u))
- Full traversal of all edges: O(V + E)
- Space: O(V + E)
