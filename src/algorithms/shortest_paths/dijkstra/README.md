# Dijkstra

Single-source shortest paths on a weighted graph with non-negative edge
weights, driven by a min-priority queue over tentative distances.

## Required API

```rust
pub fn exercise();
```

The module exposes only this scaffold entry point, which panics via `todo!`
when invoked. The contract below specifies a solver of the shape `fn
dijkstra(graph: &WeightedGraph, source: usize) -> Option<Vec<Option<u64>>>`
(distances, with parent-tracking variants as needed).

## Contract

- Precondition: all edge weights are non-negative. Negative weights break the
  greedy settlement argument and must be rejected, not silently mis-answered.
- A vertex's distance is final when it is extracted from the priority queue;
  it is never revisited afterward.
- Relaxation: for edge (u, v, w), if `dist[u] + w < dist[v]`, update `dist[v]`
  and the parent link. Lazy insertion with stale-entry skipping is acceptable
  in place of decrease-key; stale entries must be detectably skipped.
- Unreachable vertices report `None` for distance, never a garbage value.
- Parent links must reconstruct an actual shortest path from the source.
- Correct on graphs with cycles, parallel edges, and self-loops; an invalid
  source vertex yields `None` rather than panicking.
- Implement the priority queue behavior yourself (min-heap ordering via
  `Reverse` or a custom `Ord`), consistent with the curriculum's
  no-library-shortcut rule.

## Complexity Targets

- Time: O((V + E) log V) with a binary heap
- Space: O(V) for distances, parents, and the heap

## Learning Focus

Dijkstra is BFS generalized to weighted graphs: the FIFO queue becomes a
priority queue and levels become tentative distances. Implementing it teaches
the greedy exchange argument for why settled vertices are final, why that
argument collapses under negative weights, and — in Rust — how `Option<u64>`
models infinity without sentinel values and how min-heap ordering is expressed
through `Ord` implementations.

Status: scaffold — the source is a `todo!` stub; the ignored test marks the
unimplemented state.
