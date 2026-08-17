# Breadth-First Search

Level-order graph traversal that explores all vertices at distance k before
any vertex at distance k + 1, using a FIFO queue.

## Required API

```rust
pub fn exercise();
```

The module exposes only this scaffold entry point, which panics via `todo!`
when invoked. The contract below specifies a traversal of the shape `fn
breadth_first_search(graph: &AdjacencyList, source: usize) ->
Option<Vec<usize>>` (visit order, with distance/parent variants as needed).

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

## Learning Focus

BFS shows that a traversal's order is entirely a property of its frontier data
structure — swap the queue for a stack and you get DFS from the same skeleton.
The enqueue-time-marking rule is a classic correctness subtlety worth getting
wrong once and understanding. BFS is also the foundation Dijkstra generalizes:
same pattern, priority queue instead of FIFO.

Status: scaffold — the source is a `todo!` stub; the ignored test marks the
unimplemented state.
