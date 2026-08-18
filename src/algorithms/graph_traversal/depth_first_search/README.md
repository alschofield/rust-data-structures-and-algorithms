# Depth-First Search

Graph traversal that explores as far as possible along each branch before
backtracking, using recursion or an explicit stack.

## Required API

```rust
pub fn depth_first_search(
    graph: &AdjacencyList,
    source: usize,
) -> Option<Vec<usize>>;
```

The checked-in source is still the scaffold stub `pub fn exercise()`,
which panics via `todo!`; the ignored test marks the unimplemented state.
`AdjacencyList` is the graph from
`data_structures/graphs/representations/adjacency_list`.
Discovery/finish-time variants extend the same shape.

## Contract

- Visits every vertex reachable from the source exactly once; a visited set
  is mandatory to terminate on cyclic graphs.
- Both the recursive and explicit-stack forms must be understood; recursion
  depth is O(V) worst case, so deep graphs favor the explicit stack.
- Discovery/finish ordering must be consistent with DFS semantics: a vertex
  finishes only after all vertices reachable through its unvisited neighbors
  finish. This ordering is what topological sort and cycle detection build on.
- Correct on cyclic graphs, self-loops, and disconnected graphs; an invalid
  source vertex yields `None` rather than panicking.
- The graph is borrowed immutably and never modified during traversal.

## Complexity Targets

- Time: O(V + E) with an adjacency list
- Space: O(V) for the visited set plus recursion/stack depth up to O(V)
