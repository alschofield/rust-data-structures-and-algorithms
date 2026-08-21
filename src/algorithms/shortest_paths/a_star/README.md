# A-Star

Goal-directed shortest-path search that orders the frontier by
`f(n) = g(n) + h(n)`: cost so far plus a heuristic estimate to the goal.

## How It Works

Dijkstra with a sense of direction. Plain Dijkstra explores a blind circle
around the source; A* orders its frontier by f(n) = g(n) + h(n) — cost paid
so far plus a heuristic guess of the cost remaining — stretching the search
ellipse toward the goal and skipping regions that cannot help. The one law is
admissibility: the heuristic must never overestimate the true remaining cost.
Underestimating merely explores more; overestimating can commit the search to
a wrong path and return a non-optimal answer. With h(n) = 0 everywhere the
guess contributes nothing and A* degenerates into exactly Dijkstra — which
the tests check directly.

## Required API

```rust
pub fn a_star(
    graph: &WeightedGraph,
    source: usize,
    goal: usize,
    heuristic: impl Fn(usize) -> u64,
) -> Option<Vec<usize>>;
```

`WeightedGraph` is the non-negative-weight digraph this module
defines alongside the algorithm. The return value is the optimal
path, or `None` when the goal is unreachable or inputs are invalid.

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
