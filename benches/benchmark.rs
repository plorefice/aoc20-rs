use std::str::FromStr;

use aoc20::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("day1", |b| {
        b.iter(|| {
            let input = day01::parse_input(include_str!("../inputs/day01.txt"));
            day01::part_1(black_box(&input));
            day01::part_2(black_box(input));
        })
    });

    c.bench_function("day2", |b| {
        b.iter(|| {
            let input = day02::parse_input(include_bytes!("../inputs/day02.txt"));
            day02::part_1(black_box(&input));
            day02::part_2(black_box(&input));
        })
    });

    c.bench_function("day3", |b| {
        b.iter(|| {
            let input = day03::parse_input(include_str!("../inputs/day03.txt"));
            day03::part_1(black_box(&input));
            day03::part_2(black_box(&input));
        })
    });

    c.bench_function("day4", |b| {
        b.iter(|| {
            let input = day04::parse_input(include_str!("../inputs/day04.txt"));
            day04::part_1(black_box(&input));
            day04::part_2(black_box(&input));
        })
    });

    c.bench_function("day5", |b| {
        b.iter(|| {
            let input = day05::parse_input(include_str!("../inputs/day05.txt"));
            day05::part_1(black_box(&input));
            day05::part_2(black_box(input));
        })
    });

    c.bench_function("day6", |b| {
        b.iter(|| {
            let input = include_bytes!("../inputs/day06.txt");
            day06::part_1(black_box(input));
            day06::part_2(black_box(input));
        })
    });

    c.bench_function("day7", |b| {
        b.iter(|| {
            let input = day07::parse_input(include_bytes!("../inputs/day07.txt"));
            day07::part_1(black_box(&input));
            day07::part_2(black_box(&input));
        })
    });

    c.bench_function("day8", |b| {
        b.iter(|| {
            let mut input = day08::Console::from_str(include_str!("../inputs/day08.txt")).unwrap();
            day08::part_1(black_box(&mut input));
            day08::part_2(black_box(&mut input));
        })
    });

    c.bench_function("day9", |b| {
        b.iter(|| {
            let input = day09::parse_input(include_str!("../inputs/day09.txt"));
            day09::part_2(
                black_box(&input),
                black_box(day09::part_1(black_box(&input))),
            );
        })
    });

    c.bench_function("day10", |b| {
        b.iter(|| {
            let input = day10::parse_input(include_str!("../inputs/day10.txt"));
            day10::part_1(black_box(&input));
            day10::part_2(black_box(&input));
        })
    });

    c.bench_function("day11", |b| {
        b.iter(|| {
            let input = day11::parse_input(include_str!("../inputs/day11.txt"));
            day11::part_1(black_box(input.clone()));
            day11::part_2(black_box(input));
        })
    });

    c.bench_function("day12", |b| {
        b.iter(|| {
            let input = day12::parse_input(include_str!("../inputs/day12.txt"));
            day12::part_1(black_box(&input));
            day12::part_2(black_box(&input));
        })
    });

    c.bench_function("day13", |b| {
        b.iter(|| {
            let input = day13::parse_input(include_str!("../inputs/day13.txt"));
            day13::part_1(black_box(&input));
            day13::part_2(black_box(&input));
        })
    });

    c.bench_function("day14", |b| {
        b.iter(|| {
            let input = include_str!("../inputs/day14.txt");
            day14::part_1(black_box(input));
            day14::part_2(black_box(input));
        })
    });

    c.bench_function("day15", |b| {
        b.iter(|| {
            let input = day15::parse_input(include_str!("../inputs/day15.txt"));
            day15::part_1(black_box(input.clone()));
            day15::part_2(black_box(input));
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
