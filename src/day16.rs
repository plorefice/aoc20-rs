use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Input {
    rules: HashMap<String, (RangeInclusive<u64>, RangeInclusive<u64>)>,
    ticket: Vec<u64>,
    others: Vec<Vec<u64>>,
}

pub fn parse_input<S: AsRef<str>>(input: S) -> Input {
    let mut chunks = input.as_ref().split("\n\n");

    let rules = chunks
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut rule = l.split(": ");
            (
                rule.next().unwrap().to_owned(),
                rule.next()
                    .unwrap()
                    .split(" or ")
                    .map(|range| {
                        let mut sides = range.split('-');
                        sides.next().unwrap().parse().unwrap()
                            ..=sides.next().unwrap().parse().unwrap()
                    })
                    .tuples()
                    .next()
                    .unwrap(),
            )
        })
        .collect();

    let ticket = chunks
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let others = chunks
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| {
            l.split(',')
                .map(str::parse)
                .collect::<Result<_, _>>()
                .unwrap()
        })
        .collect();

    Input {
        rules,
        ticket,
        others,
    }
}

pub fn part_1(input: &mut Input) -> u64 {
    discard_invalid_tickes(input)
}

pub fn part_2(input: &mut Input) -> u64 {
    let mut possible_mappings: HashMap<&str, Vec<usize>> =
        HashMap::with_capacity(input.rules.len());

    for i in 0..input.ticket.len() {
        let mut possible_classes: HashSet<_> = input.rules.keys().collect();

        for other in &input.others {
            let v = other[i];

            possible_classes.retain(|class| {
                let (a, b) = &input.rules[class.as_str()];
                a.contains(&v) || b.contains(&v)
            })
        }

        // find a way to chain this with the above
        possible_classes.retain(|class| {
            let (a, b) = &input.rules[class.as_str()];
            a.contains(&input.ticket[i]) || b.contains(&input.ticket[i])
        });

        for class in possible_classes {
            possible_mappings
                .entry(class)
                .or_insert_with(Default::default)
                .push(i);
        }
    }

    let mut mapping = HashMap::with_capacity(possible_mappings.len());

    while !possible_mappings.is_empty() {
        let (class, index) = possible_mappings
            .iter()
            .find(|(_, v)| v.len() == 1)
            .unwrap();

        let (class, index) = (class.to_owned(), index[0]);

        mapping.insert(class, index);

        for indices in possible_mappings.values_mut() {
            indices.retain(|i| *i != index);
        }

        possible_mappings.remove(&class);
    }

    mapping
        .iter()
        .filter(|(s, _)| s.starts_with("departure"))
        .map(|(_, idx)| input.ticket[*idx as usize])
        .product::<u64>()
}

fn discard_invalid_tickes(input: &mut Input) -> u64 {
    let mut error_rate = 0;

    let Input {
        ref rules,
        ref mut others,
        ..
    } = input;

    others.retain(|other| {
        let mut retain = true;

        for v in other {
            if !rules.values().any(|(a, b)| a.contains(v) || b.contains(v)) {
                retain = false;
                error_rate += v;
            }
        }

        retain
    });

    error_rate
}

crate::solutions! {
    p1 => {
        part_1(&mut parse_input(include_str!("../inputs/day16.txt"))),
        19093
    },
    p2 => {
        {
            let mut input = parse_input(include_str!("../inputs/day16.txt"));
            part_1(&mut input);
            part_2(&mut input)
        },
        5311123569883
    }
}
