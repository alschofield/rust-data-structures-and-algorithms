# Depth-First Search

Graph traversal that explores as far as possible along each branch before
backtracking, using recursion or an explicit stack.

## Required API

```rust
pub fn exercise();
```

The module exposes only this scaffold entry point, which panics via `todo!`
when invoked. The contract below specifies a traversal of the shape `fn
depth_first_search(graph: &AdjacencyList, source: usize) ->
Option<Vec<usize>>` (visit order, with discovery/finish variants as needed).

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

## Learning Focus

DFS is the substrate for a large family of graph algorithms: topological sort,
cycle detection, connected components, and edge classification all fall out of
its discovery/finish structure. Implementing both forms teaches how the call
stack is an implicit data structure, and the iterative version in Rust
sidesteps stack-overflow risk that the recursive form carries on deep graphs.

Status: scaffold — the source is a `todo!` stub; the ignored test marks the
unimplemented state.
