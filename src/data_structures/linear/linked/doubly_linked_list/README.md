# Doubly Linked List

Generic linked list with forward and backward links per node, implemented with
owned bidirectional nodes and without `std::collections::LinkedList`.

## Required API

```rust
pub struct DoublyLinkedList<T> { /* fields private */ }

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self;
    pub fn push_front(&mut self, item: T);
    pub fn push_back(&mut self, item: T);
    pub fn pop_front(&mut self) -> Option<T>;
    pub fn pop_back(&mut self) -> Option<T>;
    pub fn get(&self, index: usize) -> Option<&T>;
    pub fn insert(&mut self, index: usize, item: T) -> Result<(), T>;
    pub fn remove(&mut self, index: usize) -> Option<T>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}

impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self;
}
```

The checked-in source is still the scaffold stub `pub fn new<T>() -> T`,
which panics via `todo!`; the ignored test marks the unimplemented state.
The `Option`/`Result` conventions follow the singly linked list.

## Contract

- Every node is reachable forward from the head and backward from the tail;
  after any mutation the `next`/`prev` pairing is consistent in both
  directions.
- Both ends support O(1) push and pop.
- Indexed operations use `[0, len)` with `insert` also accepting `len`;
  out-of-range insert returns the item via `Err(item)`, other out-of-range
  operations return `None`.
- Removing the final node leaves a valid empty list with both ends cleared.
- Dropping the list drops every node exactly once, iteratively, with no
  reference cycles leaking memory.
- Back-links cannot be owning in safe Rust; the design must choose and
  document a strategy (raw pointers with an encapsulated unsafe core, or
  `Rc<RefCell>` weak back-links) and uphold its invariants.

## Complexity Targets

- Push/pop at either end, `len`, `is_empty`: O(1)
- `get`, `insert`, `remove` by index: O(n), at most n/2 steps from the nearer
  end
- Space: O(n) nodes, two links of overhead per node
