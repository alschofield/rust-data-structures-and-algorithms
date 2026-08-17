# Counting Sort

Non-comparison integer sort that counts key occurrences, prefix-sums the
counts into positions, and places elements directly.

## Required API

```rust
pub fn exercise();
```

The module exposes only this scaffold entry point, which panics via `todo!`
when invoked. The contract below specifies a sort of the shape `fn
counting_sort(items: &mut [u32], max_key: u32)` (or an equivalent keyed
variant).

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

## Learning Focus

Counting sort is the proof that the O(n log n) lower bound applies only to
comparison sorts — with structural knowledge of the keys you can sort in
linear time. Implementing the prefix-sum placement pass teaches how counts
become positions, and preserving stability in that pass is the exact skill
radix sort depends on.

Status: scaffold — the source is a `todo!` stub; the ignored test marks the
unimplemented state.
