# Queue

Generic FIFO collection owning its elements, implemented as a growing circular
buffer.

## How It Works

First in, first out — a checkout line. Enqueue joins the back, dequeue
leaves from the front. The array-backed version keeps head and tail indexes
that wrap around the buffer (a ring), so neither operation ever shifts
elements — both are O(1). The FIFO discipline is BFS's frontier and every
producer-consumer handoff.

## Required API

```rust
pub struct Queue<T> { /* fields private */ }

impl<T> Queue<T> {
    pub fn new() -> Self;
    pub fn enqueue(&mut self, item: T);
    pub fn dequeue(&mut self) -> Option<T>;
    pub fn peek(&self) -> Option<&T>;
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
- `dequeue`/`peek` on an empty queue return `None`; `peek` never removes an
  item and neither operation panics.
- The backing store is a circular buffer: head and tail indexes wrap, and
  growth re-linearizes the wrapped contents correctly.
- Elements are owned by the queue; dropping the queue drops all remaining
  elements exactly once.
- `is_empty()` is equivalent to `len() == 0`, and `Default` matches `new`.
- No trait bounds on `T`; do not delegate to `VecDeque`.

## Complexity Targets

- `enqueue`: amortized O(1)
- `dequeue`, `peek`, `len`, `is_empty`: O(1)
- Space: O(n) contiguous
