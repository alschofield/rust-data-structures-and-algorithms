# Prefix Trie

Character-tree over strings where each root-to-node path spells a prefix,
implemented with owned prefix nodes and without a map collection.

## Required API

```rust
pub fn new<T>() -> T;
```

The module exposes only this scaffold entry point, which panics via `todo!`
when invoked. The contract below specifies a `PrefixTrie` type offering `new`,
`insert(&mut self, key: &str)`, `contains(&self, key: &str) -> bool`,
`starts_with(&self, prefix: &str) -> bool`, `remove(&mut self, key: &str) ->
bool`, and `len`.

## Contract

- Each node owns its children directly (fixed array or sorted child list —
  not `HashMap`/`BTreeMap`) plus an end-of-key flag distinguishing stored
  keys from mere prefixes.
- `insert` walks the key creating missing nodes; duplicate inserts are
  idempotent and do not double-count `len`.
- `contains` matches whole keys only; `starts_with` matches any stored key
  extending the prefix; the empty string is a valid prefix.
- `remove` clears the end flag, prunes nodes no longer on any key's path, and
  returns whether a key was actually removed; removing an absent key changes
  nothing.
- Lookup paths never mutate the trie.
- Dropping the trie drops all nodes exactly once, iteratively on deep tries.

## Complexity Targets

- `insert`, `contains`, `remove`, `starts_with`: O(m) for key/prefix length m,
  independent of the number of stored keys
- Space: O(total characters across stored keys) worst case; shared prefixes
  share nodes

## Learning Focus

The trie replaces hashing and comparison with structural position: a key's
characters are its address. Building the child storage by hand (rather than
nesting a map) forces a real representation decision with lookup/memory
trade-offs, and remove-with-pruning in owned-node Rust is a good exercise in
restructuring a tree you are borrowing your way down.

Status: scaffold — the source is a `todo!` stub; the ignored test marks the
unimplemented state.
