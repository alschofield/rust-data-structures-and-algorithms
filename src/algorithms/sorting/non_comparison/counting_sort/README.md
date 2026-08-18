# Counting Sort

Non-comparison integer sort that counts key occurrences, prefix-sums the
counts into positions, and places elements directly.

## Required API

```rust
pub fn counting_sort(items: &mut [u32], key_limit: u32);
```

The checked-in source is still the scaffold stub `pub fn exercise()`,
which panics via `todo!`; the ignored test marks the unimplemented state.
Keys must lie in `[0, key_limit)`.

## Contract

- Applies to integer keys in a known range `[0, k)`; the key range is a
  precondition, not something discovered by comparison.
- Stable: the output-placement pass must iterate the input in reverse (or use
  an equivalent scheme) so equal keys keep their original relative order.
  Stability here is what makes radix sort possible.
- Uses a counts vector of size `k` and prefix sums to compute each key's final
  output offset; no element comparisons anywhere.
- Requires an O(n + k) auxiliary allocation.
- An empty slice is a no-op; keys outside the declared range must be handled
  as a defined error, not an out-of-bounds panic in release code.
- Do not delegate to library sorts.

## Complexity Targets

- Best: O(n + k)
- Average: O(n + k)
- Worst: O(n + k)
- Space: O(n + k) auxiliary
