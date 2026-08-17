# Adjacency List

Graph representation storing, for each vertex, a list of its outgoing edges.
The standard choice for sparse graphs.

## Required API

```rust
pub fn exercise();
```

The module exposes only this scaffold entry point, which panics via `todo!`
when invoked. The contract below specifies an `AdjacencyList` type offering
construction for a fixed vertex count, `add_edge`, `has_edge`, neighbor
iteration, and vertex/edge counts, using `Option`/`Result` returns for out-of-
range vertexes.

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

## Learning Focus

The adjacency list is the representation every graph algorithm in this
curriculum assumes when it claims O(V + E): work scales with edges that exist,
not all possible pairs. In Rust, exposing neighbor iteration as a borrowing
iterator is the idiomatic design point — it teaches how lifetimes tie an
iterator to the structure it reads without copying edge lists.

Status: scaffold — the source is a `todo!` stub; the ignored test marks the
unimplemented state.
