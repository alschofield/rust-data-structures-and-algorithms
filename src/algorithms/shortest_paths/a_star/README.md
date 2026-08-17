# A-Star

Goal-directed shortest-path search that orders the frontier by
`f(n) = g(n) + h(n)`: cost so far plus a heuristic estimate to the goal.

## Required API

```rust
pub fn exercise();
```

The module exposes only this scaffold entry point, which panics via `todo!`
when invoked. The contract below specifies a solver of the shape `fn
a_star(graph: &WeightedGraph, source: usize, goal: usize, heuristic: impl
Fn(usize) -> u64) -> Option<Vec<usize>>` (the path, or `None` when
unreachable).

## Contract

- The heuristic must be admissible (never overestimates the true remaining
  cost) for the returned path to be optimal; a consistent (monotone)
  heuristic additionally guarantees no vertex needs re-expansion.
- With a zero heuristic, the algorithm must degenerate to exactly Dijkstra's
  behavior.
- Frontier is a min-priority queue keyed on `f`; ties are resolved
  deterministically.
- Precondition: non-negative edge weights, as with Dijkstra.
- Terminates with the optimal path when the goal is extracted from the
  frontier (given an admissible heuristic), or returns `None` when the
  frontier empties.
- Parent links must reconstruct the returned path; invalid source or goal
  vertexes yield `None` rather than panicking.

## Complexity Targets

- Time: worst case O((V + E) log V), same as Dijkstra (an uninformative
  heuristic gives no pruning); a strong admissible heuristic prunes most of
  the graph in practice
- Space: O(V) for scores, parents, and the frontier

## Learning Focus

A-star shows how domain knowledge slots into a general algorithm without
breaking its guarantees: the heuristic reshapes exploration order while
admissibility preserves optimality. Implementing it clarifies the
admissible/consistent distinction and why the goal test must happen at
extraction, not at edge relaxation — testing early sacrifices optimality.
Passing the heuristic as a generic `Fn` parameter is idiomatic Rust for
pluggable strategies.

Status: scaffold — the source is a `todo!` stub; the ignored test marks the
unimplemented state.
