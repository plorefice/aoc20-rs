use aoc20_rs::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day1(c: &mut Criterion) {
    c.bench_function("day1", |b| {
        b.iter(|| {
            let input = day1::parse_input(include_str!("../inputs/day1.txt"));
            day1::part_1(black_box(&input));
            day1::part_2(black_box(input));
        })
    });
}

pub fn day2(c: &mut Criterion) {
    c.bench_function("day2", |b| {
        b.iter(|| {
            let input = day2::parse_input(include_bytes!("../inputs/day2.txt"));
            day2::part_1(black_box(&input));
            day2::part_2(black_box(&input));
        })
    });
}

pub fn day3(c: &mut Criterion) {
    c.bench_function("day3", |b| {
        b.iter(|| {
            let input = day3::parse_input(include_str!("../inputs/day3.txt"));
            day3::part_1(black_box(&input));
            day3::part_2(black_box(&input));
        })
    });
}

pub fn day4(c: &mut Criterion) {
    c.bench_function("day4", |b| {
        b.iter(|| {
            let input = day4::parse_input(include_str!("../inputs/day4.txt"));
            day4::part_1(black_box(&input));
            day4::part_2(black_box(&input));
        })
    });
}

pub fn day5(c: &mut Criterion) {
    c.bench_function("day5", |b| {
        b.iter(|| {
            let input = day5::parse_input(include_str!("../inputs/day5.txt"));
            day5::part_1(black_box(&input));
            day5::part_2(black_box(input));
        })
    });
}

criterion_group!(benches, day1, day2, day3, day4, day5);
criterion_main!(benches);
