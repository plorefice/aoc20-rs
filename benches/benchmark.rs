use aoc20_rs::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day_1(c: &mut Criterion) {
    c.bench_function("day_1", |b| {
        b.iter(|| {
            let input = day1::parse_input(include_str!("../inputs/day1.txt"));
            day1::part_1(black_box(&input));
            day1::part_2(black_box(input));
        })
    });
}

pub fn day_2(c: &mut Criterion) {
    c.bench_function("day_2", |b| {
        b.iter(|| {
            let input = day2::parse_input(include_str!("../inputs/day2.txt"));
            day2::part_1(black_box(&input));
            day2::part_2(black_box(&input));
        })
    });
}

pub fn day_3(c: &mut Criterion) {
    c.bench_function("day_3", |b| {
        b.iter(|| {
            let input = day3::parse_input(include_str!("../inputs/day3.txt"));
            day3::part_1(black_box(&input));
            day3::part_2(black_box(&input));
        })
    });
}

pub fn day_4(c: &mut Criterion) {
    c.bench_function("day_4", |b| {
        b.iter(|| {
            let input = day4::parse_input(include_str!("../inputs/day4.txt"));
            day4::part_1(black_box(&input));
            day4::part_2(black_box(&input));
        })
    });
}

pub fn day_5(c: &mut Criterion) {
    c.bench_function("day_5", |b| {
        b.iter(|| {
            let input = day5::parse_input(include_str!("../inputs/day5.txt"));
            day5::part_1(black_box(&input));
            day5::part_2(black_box(input));
        })
    });
}

criterion_group!(benches, day_1, day_2, day_3, day_4, day_5);
criterion_main!(benches);
