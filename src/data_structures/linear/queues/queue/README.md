# Queue

Generic FIFO collection owning its elements, implemented as a growing circular
buffer.

## Required API

```rust
pub struct Queue<T> { /* fields private */ }

impl<T> Queue<T> {
    pub fn new() -> Self;
    pub fn enqueue(&mut self, item: T);
    pub fn dequeue(&mut self) -> Option<T>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}

impl<T> Default for Queue<T> {
    fn default() -> Self;
}
```

## Contract

- `enqueue` adds at the back; `dequeue` removes and returns the oldest item by
  value, preserving strict FIFO order.
- `dequeue` on an empty queue returns `None`; it never panics.
- The backing store is a circular buffer: head and tail indexes wrap, and
  growth re-linearizes the wrapped contents correctly.
- Elements are owned by the queue; dropping the queue drops all remaining
  elements exactly once.
- `is_empty()` is equivalent to `len() == 0`, and `Default` matches `new`.
- No trait bounds on `T`; do not delegate to `VecDeque`.

## Complexity Targets

- `enqueue`: amortized O(1)
- `dequeue`, `len`, `is_empty`: O(1)
- Space: O(n) contiguous
