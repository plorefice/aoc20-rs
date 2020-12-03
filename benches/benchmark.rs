use aoc20_rs::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day_1(c: &mut Criterion) {
    let input = day1::parse_input(include_str!("../inputs/day1.txt"));

    c.bench_function("day_1", |b| {
        b.iter_with_setup(
            || input.clone(),
            |data| {
                day1::part_1(black_box(&data));
                day1::part_2(black_box(data));
            },
        )
    });
}

pub fn day_2(c: &mut Criterion) {
    let input = day2::parse_input(include_str!("../inputs/day2.txt"));

    c.bench_function("day_2", |b| {
        b.iter(|| {
            day2::part_1(black_box(&input));
            day2::part_2(black_box(&input));
        })
    });
}

pub fn day_3(c: &mut Criterion) {
    let input = day3::parse_input(include_str!("../inputs/day3.txt"));

    c.bench_function("day_3", |b| {
        b.iter(|| {
            day3::part_1(black_box(&input));
            day3::part_2(black_box(&input));
        })
    });
}

criterion_group!(benches, day_1, day_2, day_3);
criterion_main!(benches);
