# Breadth-First Search

Level-order graph traversal that explores all vertices at distance k before
any vertex at distance k + 1, using a FIFO queue.

## How It Works

Ripple outward. Starting from the source, visit everything one edge away,
then everything two edges away, ring by ring. The FIFO queue is what creates
that order: vertices enter the frontier in discovery order and leave it in
the same order, so distance-k vertices are fully processed before any
distance-k+1 vertex. That ripple property is why BFS computes minimum-hop
distances on unweighted graphs and why its parent links form a shortest-path
tree. The classic bug: a vertex must be marked visited when it is enqueued,
not when dequeued — otherwise cycles push the same vertex into the queue
repeatedly through different neighbors.

## Required API

```rust
pub fn breadth_first_search(
    graph: &AdjacencyList,
    source: usize,
) -> Option<Vec<usize>>;
```

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
