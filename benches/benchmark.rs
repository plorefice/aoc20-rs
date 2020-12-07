use aoc20::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("day1", |b| {
        b.iter(|| {
            let input = day1::parse_input(include_str!("../inputs/day1.txt"));
            day1::part_1(black_box(&input));
            day1::part_2(black_box(input));
        })
    });

    c.bench_function("day2", |b| {
        b.iter(|| {
            let input = day2::parse_input(include_bytes!("../inputs/day2.txt"));
            day2::part_1(black_box(&input));
            day2::part_2(black_box(&input));
        })
    });

    c.bench_function("day3", |b| {
        b.iter(|| {
            let input = day3::parse_input(include_str!("../inputs/day3.txt"));
            day3::part_1(black_box(&input));
            day3::part_2(black_box(&input));
        })
    });

    c.bench_function("day4", |b| {
        b.iter(|| {
            let input = day4::parse_input(include_str!("../inputs/day4.txt"));
            day4::part_1(black_box(&input));
            day4::part_2(black_box(&input));
        })
    });

    c.bench_function("day5", |b| {
        b.iter(|| {
            let input = day5::parse_input(include_str!("../inputs/day5.txt"));
            day5::part_1(black_box(&input));
            day5::part_2(black_box(input));
        })
    });

    c.bench_function("day6", |b| {
        b.iter(|| {
            let input = include_bytes!("../inputs/day6.txt");
            day6::part_1(black_box(input));
            day6::part_2(black_box(input));
        })
    });

    c.bench_function("day7", |b| {
        b.iter(|| {
            let input = day7::parse_input(include_bytes!("../inputs/day7.txt"));
            day7::part_1(black_box(&input));
            day7::part_2(black_box(&input));
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
