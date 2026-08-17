# Adjacency Matrix

Graph representation storing edge presence (or weight) in a V x V grid,
indexed by vertex pair. The standard choice for dense graphs.

## Required API

```rust
pub fn exercise();
```

The module exposes only this scaffold entry point, which panics via `todo!`
when invoked. The contract below specifies an `AdjacencyMatrix` type offering
construction for a fixed vertex count, `add_edge`, `remove_edge`, `has_edge`,
neighbor iteration, and vertex/edge counts, using `Option`/`Result` returns
for out-of-range vertexes.

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

## Learning Focus

The matrix is the memory-for-speed extreme of graph representation: O(1) edge
checks bought with O(V^2) space and O(V) neighbor scans. Implementing both
representations against the same conceptual interface makes concrete how
representation choice changes algorithm cost — BFS on a matrix is O(V^2), not
O(V + E) — and where the matrix earns its keep (dense graphs, Floyd-Warshall,
constant-time edge queries).

Status: scaffold — the source is a `todo!` stub; the ignored test marks the
unimplemented state.
