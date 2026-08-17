use std::time::{Duration, Instant};

const SAMPLE_COUNT: usize = 21;

fn median(mut samples: Vec<Duration>) -> Duration {
    samples.sort_unstable();
    samples[samples.len() / 2]
}

fn measure<F>(name: &str, iterations: usize, mut operation: F)
where
    F: FnMut(),
{
    let mut samples = Vec::with_capacity(SAMPLE_COUNT);

    for _ in 0..SAMPLE_COUNT {
        let started = Instant::now();
        for _ in 0..iterations {
            operation();
        }
        samples.push(started.elapsed());
    }

    let median = median(samples);
    let nanoseconds_per_operation = median.as_secs_f64() * 1_000_000_000.0 / iterations as f64;
    println!("{name}: {SAMPLE_COUNT} samples x {iterations} operations");
    println!("  median ns/op: {nanoseconds_per_operation:.2}");
}

fn main() {
    let Some(module) = std::env::args().nth(1) else {
        eprintln!("usage: cargo run --release --bin benchmark -- <module>");
        eprintln!("modules become available after their implementations are complete");
        return;
    };

    match module.as_str() {
        // TODO: Add setup, operation, verification, and teardown for each completed module.
        "algorithms/searching/linear-search"
        | "algorithms/searching/binary-search"
        | "algorithms/sorting/comparison/bubble-sort"
        | "data-structures/linear/stacks/stack"
        | "data-structures/linear/queues/queue"
        | "data-structures/linear/linked/singly-linked-list"
        | "data-structures/linear/arrays/dynamic-array"
        | "data-structures/associative/hash-tables/separate-chaining"
        | "data-structures/trees/binary-search-trees/binary-search-tree" => {
            eprintln!("{module} benchmark is scaffolded but not implemented yet");
        }
        _ => eprintln!("unknown module: {module}"),
    }

    // Keeps measure compiled and ready for module-specific benchmark wiring.
    let _ = measure::<fn()>;
}
