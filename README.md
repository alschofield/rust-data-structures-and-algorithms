# Data Structures and Algorithms in Rust

A Rust counterpart to the C data-structures-and-algorithms portfolio. The two
repositories follow the same problem sequence and behavioral contracts while
using each language's appropriate ownership model.

> Brought to you by the offices of SchoTech: we bring you something you already
> have and will not need in the future, in a language you have already seen and
> also will not need in the future.

## Learning Model

Implement each module from first principles. Do not delegate the core data
structure or algorithm to `Vec`, `VecDeque`, `LinkedList`, `HashMap`,
`BTreeMap`, `BinaryHeap`, or library sorting/search methods. Those are useful
production tools; this repository exists to expose the implementation tradeoffs
they normally hide.

Rust should not be written as C with different syntax. The equivalent lessons
are still present, but move through Rust concepts:

| C focus | Rust counterpart |
| --- | --- |
| `malloc`/`free`, ownership comments | ownership, `Drop`, and `Option` |
| nullable pointers | `Option<T>` and `Option<&T>` |
| function pointers for comparison | `Ord`, `Eq`, and `Hash` trait bounds |
| linked-node pointers | `Box`, borrowing, and safe mutation constraints |
| output parameters | return values such as `Option<T>` and `Result<(), T>` |
| allocation failure handling | ordinary allocation behavior plus deliberate API errors |

## Module Sequence

| Module | C status | Rust status | Primary lesson |
| --- | --- | --- | --- |
| Stack | Complete | Scaffold | LIFO, growth, ownership on pop |
| Queue | Complete | Scaffold | Circular buffers and logical order |
| Singly linked list | Complete | Scaffold | `Box`, links, and borrow boundaries |
| Dynamic array | Complete | Scaffold | Contiguous storage and shifting |
| Hash table | Complete, needs rehashing | Scaffold | hashing, collisions, resizing |
| Binary search tree | In progress | Scaffold | ordered search and recursive traversal |
| Linear search | Contract/test scaffold | Scaffold | invariants and unsorted lookup |
| Binary search | Contract/test scaffold | Scaffold | ranges and ordered lookup |
| Bubble, selection, insertion sort | Contract/test scaffold | Scaffold | swaps, prefixes, stability |
| Merge, quick sort | Contract/test scaffold | Scaffold | recursion, partitions, temporary storage |
| Doubly linked list | Planned | Planned | bidirectional links and removal |
| Heap / priority queue | Planned | Planned | array-backed trees and heap invariants |
| Trie | Planned | Planned | prefix lookup and sparse children |
| Graph | Planned | Planned | adjacency representations and ownership |
| BFS / DFS / Dijkstra | Planned | Planned | traversal, queues/stacks, shortest paths |

## Layout

```text
src/        One Rust module per data structure or algorithm
tests/      Ignored acceptance tests enabled when a module is implemented
benches/    Dependency-free benchmark runner and benchmark design notes
```

## Verification

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
cargo run --release --bin benchmark -- <module>
```

Acceptance tests are marked `#[ignore]` while their module contains `todo!`.
Enable the relevant test only after the implementation is ready; run all
accepted tests with:

```bash
cargo test -- --ignored
```

## Benchmarking

The benchmark runner uses `std::time::Instant` and reports repeated batch
measurements. It excludes process startup, setup, verification, and teardown
from timed operation batches. Benchmarks are evidence for a machine and build,
not proof of Big-O complexity.

Use `--release` for meaningful measurements. Keep each benchmark dependency-free
and provide identical input distributions across algorithms: random, sorted,
reversed, and duplicate-heavy for sorting; present and absent keys for search.

## Source Material

- [https://www.youtube.com/watch?v=6z2W06cmpmE](https://www.youtube.com/watch?v=6z2W06cmpmE)
- [https://www.youtube.com/watch?v=We2om9wO-bI](https://www.youtube.com/watch?v=We2om9wO-bI)
- The APIs, behavioral contracts, tests, benchmark harness, and README material
  are authored directly in these portfolios; no other external technical source
  was used or attributed during their creation.
