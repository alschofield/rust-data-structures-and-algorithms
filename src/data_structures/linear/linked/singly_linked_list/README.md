# Singly Linked List

Generic head-only linked list of owned boxed nodes with indexed operations.

## Required API

```rust
pub struct SinglyLinkedList<T> { /* fields private */ }

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self;
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

- `insert`/`remove` at index 0: O(1)
- `get`, `insert`, `remove` at index i: O(i), worst O(n)
- `len`, `is_empty`: O(1)
- Space: O(n) nodes, one pointer of overhead per node

## Learning Focus

The singly linked list is the standard crucible for Rust ownership: each node
owns the next, so splicing requires moving links out with `Option::take` and
putting them back without ever holding two mutable paths to the same node.
It also surfaces the non-obvious need for an iterative `Drop` and why linked
lists that are trivial in C demand real design in safe Rust.

Status: scaffold — the source is a `todo!` stub; the ignored test marks the
unimplemented state.
