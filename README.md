# Data Structures and Algorithms in Rust

The `src/` module hierarchy is the curriculum. Rust uses underscore module names while preserving the requested curriculum grouping: `data_structures` and `algorithms`.

## Taxonomy

```text
data_structures/linear/{arrays/dynamic_array,stacks/stack,queues/queue,linked/{singly_linked_list,doubly_linked_list}}
data_structures/associative/hash_tables/separate_chaining
data_structures/trees/{binary_search_trees/binary_search_tree,heaps/binary_heap,tries/prefix_trie}
data_structures/graphs/{representations/{adjacency_list,adjacency_matrix},disjoint_sets/union_find}
algorithms/searching/{linear_search,binary_search}
algorithms/sorting/{comparison/{bubble_sort,selection_sort,insertion_sort,merge_sort,quick_sort,heap_sort},non_comparison/{counting_sort,radix_sort}}
algorithms/graph_traversal/{breadth_first_search,depth_first_search}
algorithms/shortest_paths/{dijkstra,a_star}
```

Each leaf has `mod.rs`, `tests.rs`, and a README contract. Every module is a scaffold: topic tests are ignored and implementation entry points panic via `todo!` only if invoked. No topic has a benchmark implementation; `src/bin/benchmark.rs` is a harness scaffold.

## Verification

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
```

Implement the structures and algorithms directly rather than using standard collection, search, or sort implementations.
