# Doubly Linked List

Generic linked list with forward and backward links per node, implemented with
owned bidirectional nodes and without `std::collections::LinkedList`.

## Required API

```rust
pub fn new<T>() -> T;
```

The module exposes only this scaffold entry point, which panics via `todo!`
when invoked. The contract below specifies a `DoublyLinkedList<T>` type
offering `new`, front/back push and pop, indexed `get`/`insert`/`remove`,
`len`, and `is_empty`, following the same `Option`/`Result` conventions as the
singly linked list.

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

## Learning Focus

The doubly linked list is where Rust's ownership model visibly bites: two
owners per node is impossible, so back-edges force an explicit aliasing
strategy. Implementing it teaches the trade-offs between an unsafe-core/safe-
API design and `Rc<RefCell>` interior mutability, and why encapsulating a
small unsafe region behind a sound interface is a core systems-Rust skill.

Status: scaffold — the source is a `todo!` stub; the ignored test marks the
unimplemented state.
