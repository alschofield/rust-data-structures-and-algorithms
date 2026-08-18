# Breadth-First Search

Level-order graph traversal that explores all vertices at distance k before
any vertex at distance k + 1, using a FIFO queue.

## Required API

```rust
pub fn breadth_first_search(
    graph: &AdjacencyList,
    source: usize,
) -> Option<Vec<usize>>;
```

The checked-in source is still the scaffold stub `pub fn exercise()`,
which panics via `todo!`; the ignored test marks the unimplemented state.
`AdjacencyList` is the graph from
`data_structures/graphs/representations/adjacency_list`. Distance and
parent-tracking variants extend the same shape.

## Contract

- Uses a FIFO queue as the frontier; the queue discipline is what produces
  level order.
- A vertex is marked visited when enqueued, not when dequeued; otherwise the
  same vertex can enter the queue multiple times.
- Visits every vertex reachable from the source exactly once; unreachable
  vertices are never visited.
- On an unweighted graph, computed distances are minimum edge counts and
  recorded parent links form a valid shortest-path tree.
- Correct on cyclic graphs, self-loops, and disconnected graphs; an invalid
  source vertex yields `None` rather than panicking.
- The graph is borrowed immutably and never modified during traversal.

## Complexity Targets

- Time: O(V + E) with an adjacency list
- Space: O(V) for the visited set, queue, and parent storage
