# Stack

Generic LIFO collection owning its elements, implemented without delegating to
`Vec` methods.

## Required API

```rust
pub struct Stack<T> { /* fields private */ }

impl<T> Stack<T> {
    pub fn new() -> Self;
    pub fn push(&mut self, item: T);
    pub fn pop(&mut self) -> Option<T>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}

impl<T> Default for Stack<T> {
    fn default() -> Self;
}
```

## Contract

- `push` places an item on top; `pop` removes and returns the most recently
  pushed item by value, transferring ownership back to the caller.
- `pop` on an empty stack returns `None`; it never panics.
- Elements are owned by the stack; dropping the stack drops all remaining
  elements exactly once.
- `is_empty()` is equivalent to `len() == 0`, and `Default` matches `new`.
- No trait bounds on `T`: the stack must work for any owned type.
- Implement the backing growth directly; do not delegate to `Vec::push`/`pop`.

## Complexity Targets

- `push`: amortized O(1)
- `pop`, `len`, `is_empty`: O(1)
- Space: O(n) contiguous
