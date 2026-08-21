# Bubble Sort

Comparison sort that repeatedly sweeps the slice, swapping adjacent
out-of-order pairs until a full pass makes no swaps.

## How It Works

Repeated neighborhood sweeps. Walk the array comparing adjacent pairs and
swapping any pair that is out of order; each full pass carries the largest
remaining element to the end of the unsorted region like a bubble rising.
The shrinking boundary exploits that: after pass k the last k slots are
final, so the next pass stops earlier. A pass with zero swaps proves the
array is sorted, which is the required early exit and the O(n) best case.
Swapping only on strictly-greater keeps equal elements in their original
order (stable).

## Required API

```rust
pub fn bubble_sort<T: Ord>(items: &mut [T]);
```


## Contract

- Sorts the slice in place into ascending order under `Ord`.
- Stable: equal elements keep their original relative order (adjacent swaps
  only, never swap on equality).
- Must implement the early-exit optimization: a pass with zero swaps
  terminates the sort.
- Empty and single-element slices are no-ops; there is no panic path.
- After each pass `k`, the largest `k` elements occupy their final positions
  at the tail.
- Do not delegate to `slice::sort` or `slice::sort_unstable`.

## Complexity Targets

- Best: O(n) (already sorted input, early exit after one pass)
- Average: O(n^2)
- Worst: O(n^2) (reverse-sorted input)
- Space: O(1), in place
