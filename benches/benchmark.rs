use aoc20_rs::day2;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("day_2", |b| {
        let pwds = day2::parse_input(include_str!("../inputs/day2.txt"));

        b.iter(move || {
            day2::part_1(black_box(&pwds));
            day2::part_2(black_box(&pwds));
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
