# Benchmark Design

Add one benchmark function per completed module to `src/bin/benchmark.rs`.

Every benchmark must:

1. Build inputs before timing starts.
2. Run one repeated operation inside the timed region.
3. Verify the result after timing ends.
4. Release/drop state after verification.
5. Use enough operations to overcome timer noise.
6. Compare several input sizes before making a complexity claim.

Do not benchmark an unfinished `todo!` implementation. Use `cargo run --release`
to ensure the optimizer is enabled.
