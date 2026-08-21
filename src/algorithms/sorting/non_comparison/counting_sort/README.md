# Counting Sort

Non-comparison integer sort that counts key occurrences, prefix-sums the
counts into positions, and places elements directly.

## How It Works

No comparisons at all. Keys are small integers in a known range [0, k), so
counting replaces comparing: tally how many of each key exist, prefix-sum the
tallies so each key knows where its block of the output starts, then place
every element directly into its computed slot. Because nothing is compared,
the O(n log n) lower bound on comparison sorts does not apply — the cost is
O(n + k). The placement pass iterates the input in reverse so equal keys keep
their original order; that stability is not a nicety, it is the property
radix sort is built on.

## Required API

```rust
pub fn counting_sort(items: &mut [u32], key_limit: u32);
```

Keys must lie in `[0, key_limit)`.

## Contract

- Applies to integer keys in a known range `[0, k)` (or an offset range); the
  key range is a precondition, not something discovered by comparison.
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
