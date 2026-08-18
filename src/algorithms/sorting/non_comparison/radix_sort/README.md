# Radix Sort

Non-comparison integer sort that sorts by one digit at a time using a stable
counting sort per digit, least significant digit first.

## Required API

```rust
pub fn radix_sort(items: &mut [u32]);
```

The checked-in source is still the scaffold stub `pub fn exercise()`,
which panics via `todo!`; the ignored test marks the unimplemented state.

## Contract

- LSD order: process digits from least to most significant; correctness
  depends on it.
- Each per-digit pass must be a stable sort (counting sort by that digit);
  an unstable inner pass breaks the whole algorithm.
- Digit extraction uses a fixed radix (for example base 256 via byte masks);
  the digit count `d` is determined by the key width, not by comparisons.
- Overall the sort is stable: equal keys keep their original relative order.
- Uses an O(n + k) auxiliary buffer per pass, reusable across passes.
- An empty slice is a no-op; there is no panic path.
- Do not delegate to library sorts.

## Complexity Targets

- Best: O(d(n + k)) for d digits in radix k
- Average: O(d(n + k))
- Worst: O(d(n + k))
- Space: O(n + k) auxiliary
