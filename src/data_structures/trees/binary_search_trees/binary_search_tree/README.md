# Binary Search Tree

Generic unbalanced ordered collection of owned values, ordered by the `Ord`
implementation of the element type.

## Required API

```rust
pub struct BinarySearchTree<T> { /* fields private */ }

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> Self;
    pub fn insert(&mut self, item: T) -> Result<(), T>;
    pub fn get(&self, key: &T) -> Option<&T>;
    pub fn remove(&mut self, key: &T) -> Option<T>;
    pub fn contains(&self, key: &T) -> bool;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}

impl<T: Ord> Default for BinarySearchTree<T> {
    fn default() -> Self;
}
```

## Contract

- BST invariant: for every node, all values in the left subtree order strictly
  before it and all values in the right subtree strictly after, under `Ord`.
- Duplicates are rejected: inserting a value equal to a stored one fails with
  `Err(item)`, returning ownership and preserving the first stored value.
- `get` returns a reference to the stored value equal to the key; `remove`
  detaches and returns the stored value by value.
- `remove` must handle leaf nodes, single-child nodes, two-child nodes
  (replace with the in-order successor), and the root.
- In-order traversal of the tree yields values in strictly increasing order.
- Dropping the tree drops every node iteratively (no recursion-depth panic on
  degenerate chains) and each value exactly once. Do not delegate to
  `std::collections::BTreeMap`/`BTreeSet`.

## Complexity Targets

- Balanced shape: `insert`, `get`, `remove`, `contains`: O(log n)
- This unbalanced BST, degenerate (sorted-insert) shape: O(n) worst case
- In-order traversal: O(n)
- Space: O(n) nodes plus O(height) working space for traversal/removal
