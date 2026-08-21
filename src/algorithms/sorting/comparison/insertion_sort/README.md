# Insertion Sort

Comparison sort that grows a sorted prefix by shifting each new element left
until it reaches its correct position.

## How It Works

Sorting cards in a hand. An invisible line divides the array: left of it is
sorted, right of it is raw. Each round takes the first raw element and walks
it leftward through the sorted region, shifting strictly-greater elements one
slot right, and drops it in front of the first element that is not greater.
Stopping at the first non-greater element is what preserves the order of
equals (stability), and it is why already-sorted input costs one comparison
per element: each new element only travels as far as it is displaced.

## Required API

```rust
pub fn insertion_sort<T: Ord>(items: &mut [T]);
```


## Contract

- Sorts the slice in place into ascending order under `Ord`.
- Stable: shift while strictly greater, insert after equal elements, so equal
  elements keep their original relative order.
- Adaptive: nearly sorted input costs close to O(n); each element moves only
  as far as its displacement.
- Empty and single-element slices are no-ops; there is no panic path.
- Invariant: before processing index `i`, the range `[0, i)` is sorted.
- Do not delegate to `slice::sort` or `slice::sort_unstable`.

## Complexity Targets

- Best: O(n) (already sorted input, one comparison per element)
- Average: O(n^2)
- Worst: O(n^2) (reverse-sorted input)
- Space: O(1), in place
