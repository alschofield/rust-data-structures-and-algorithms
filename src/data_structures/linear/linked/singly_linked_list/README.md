# Singly Linked List

Generic head-only linked list of owned boxed nodes with indexed operations.

## How It Works

A chain of nodes, each holding a value and a pointer to the next. The list
holds only the head, so the front is O(1) and everything else is a walk —
push back must traverse all n nodes because nothing remembers the tail.
Insertion and removal never shift elements; they re-point two pointers.
The trade against the dynamic array: cheap splicing, but every step is a
dependent pointer load with no cache locality.

## Required API

```rust
pub struct SinglyLinkedList<T> { /* fields private */ }

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self;
    pub fn push_front(&mut self, item: T);
    pub fn push_back(&mut self, item: T);
    pub fn pop_front(&mut self) -> Option<T>;
    pub fn pop_back(&mut self) -> Option<T>;
    pub fn insert(&mut self, index: usize, item: T) -> Result<(), T>;
    pub fn get(&self, index: usize) -> Option<&T>;
    pub fn remove(&mut self, index: usize) -> Option<T>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}

impl<T> Default for SinglyLinkedList<T> {
    fn default() -> Self;
}
```

## Contract

- Nodes are owned via `Box`, linked as `Option<Box<Node<T>>>` from a single
  head pointer; each node owns its successor.
- `push_front`/`pop_front` operate at the held head pointer; `push_back` and
  `pop_back` traverse from that head because the list deliberately has no tail.
- Valid element indexes are `[0, len)`; `insert` also accepts `len` to append.
- `insert` at an out-of-range index fails with `Err(item)`, returning
  ownership of the rejected item; the list is unchanged.
- `remove` returns the removed element by value or `None` out of range;
  removing the final node leaves a valid empty list.
- Dropping the list drops every node without recursion-depth blowups on long
  lists (iterative drop).
- `is_empty()` is equivalent to `len() == 0`, and `Default` matches `new`.
- No trait bounds on `T`; do not delegate to `std::collections::LinkedList`.

## Complexity Targets

- `push_front`, `pop_front`, `len`, `is_empty`: O(1)
- `push_back`, `pop_back`, `get`, `insert`, `remove`: O(n)
- `len`, `is_empty`: O(1)
- Space: O(n) nodes, one pointer of overhead per node
