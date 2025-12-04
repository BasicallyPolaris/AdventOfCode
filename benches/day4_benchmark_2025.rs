use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[path = "../src/2025/mod.rs"]
mod y2025;
use y2025::day4::task_two;
use y2025::day4::task_two_alt;

fn criterion_benchmark(c: &mut Criterion) {
    // Setup data once
    let contents =
        std::fs::read_to_string("src/2025/input/day4.input").expect("Failed to read file");
    let diagram_lines: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut group = c.benchmark_group("Task 2 Comparison");

    group.bench_function("task_two", |b| {
        b.iter(|| {
            // use black_box to prevent compiler from optimizing away the result
            task_two(black_box(&diagram_lines))
        })
    });

    group.bench_function("task_two_alt", |b| {
        b.iter(|| task_two_alt(black_box(&diagram_lines)))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
