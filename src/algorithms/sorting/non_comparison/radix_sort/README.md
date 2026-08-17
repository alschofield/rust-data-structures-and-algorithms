# Radix Sort

Non-comparison integer sort that sorts by one digit at a time using a stable
counting sort per digit, least significant digit first.

## Required API

```rust
pub fn exercise();
```

The module exposes only this scaffold entry point, which panics via `todo!`
when invoked. The contract below specifies a sort of the shape `fn
radix_sort(items: &mut [u32])`.

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

## Learning Focus

Radix sort shows how a stable subroutine composes into a bigger algorithm:
LSD ordering works only because earlier (less significant) passes are never
undone by later stable passes. Implementing it makes the radix/digit-count
trade-off concrete — larger radix means fewer passes but bigger count arrays —
and bit-mask digit extraction is a clean exercise in integer manipulation.

Status: scaffold — the source is a `todo!` stub; the ignored test marks the
unimplemented state.
