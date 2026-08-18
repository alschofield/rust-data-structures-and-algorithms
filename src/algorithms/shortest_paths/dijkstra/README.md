# Dijkstra

Single-source shortest paths on a weighted graph with non-negative edge
weights, driven by a min-priority queue over tentative distances.

## Required API

```rust
pub fn dijkstra(graph: &WeightedGraph, source: usize) -> Option<Vec<Option<u64>>>;
```

The checked-in source is still the scaffold stub `pub fn exercise()`,
which panics via `todo!`; the ignored test marks the unimplemented state.
`WeightedGraph` is the non-negative-weight digraph this module
defines alongside the algorithm. Unreachable vertexes report `None`;
parent-tracking variants extend the same shape.

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
