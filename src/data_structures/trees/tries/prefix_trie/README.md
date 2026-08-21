# Prefix Trie

Character-tree over strings where each root-to-node path spells a prefix,
implemented with owned prefix nodes and without a map collection.

## How It Works

Strings stored as a tree of characters: the root is the empty string, each
edge adds one letter, and a root-to-node path spells a prefix. "car" and
"carton" share the c-a-r path and diverge after it, so shared prefixes are
stored exactly once. An end-of-key flag on each node distinguishes a stored
word from a mere waypoint — "car" can be a real entry while "cart" is only a
path on the way to "carton".

Every operation costs O(m) in the key's length, independent of how many keys
the trie holds — the property neither a hash table nor a BST can offer, and
what makes prefix queries (starts_with) natural. The delicate operation is
remove: clear the flag, then prune nodes that no longer lead to any stored
key without cutting a branch another key still needs.

## Required API

```rust
pub struct PrefixTrie { /* fields private */ }

impl PrefixTrie {
    pub fn new() -> Self;
    pub fn insert(&mut self, key: &str);
    pub fn contains(&self, key: &str) -> bool;
    pub fn starts_with(&self, prefix: &str) -> bool;
    pub fn remove(&mut self, key: &str) -> bool;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}

impl Default for PrefixTrie {
    fn default() -> Self;
}
```

The checked-in source is still the scaffold stub `pub fn new<T>() -> T`,
which panics via `todo!`; the ignored test marks the unimplemented state.

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
