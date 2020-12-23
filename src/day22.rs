use std::{
    collections::{hash_map::DefaultHasher, HashSet, VecDeque},
    hash::{Hash, Hasher},
};

use itertools::Itertools;

pub fn parse_input<S: AsRef<str>>(input: S) -> (VecDeque<u64>, VecDeque<u64>) {
    input
        .as_ref()
        .split("\n\n")
        .map(|input| {
            input
                .lines()
                .skip(1)
                .map(str::parse)
                .collect::<Result<_, _>>()
                .unwrap()
        })
        .next_tuple()
        .unwrap()
}

pub fn part_1(input: (VecDeque<u64>, VecDeque<u64>)) -> u64 {
    play_game(input, false).1
}

pub fn part_2(input: (VecDeque<u64>, VecDeque<u64>)) -> u64 {
    play_game(input, true).1
}

fn play_game(input: (VecDeque<u64>, VecDeque<u64>), rec: bool) -> (bool, u64) {
    let (mut d1, mut d2) = input;
    let mut seen = HashSet::with_capacity(2048);

    // Store the hash of the state rather than the state itself for a major speedup
    let hash_fn = |d1: &VecDeque<u64>, d2: &VecDeque<u64>| -> u64 {
        let mut hasher = DefaultHasher::new();
        d1.hash(&mut hasher);
        d2.hash(&mut hasher);
        hasher.finish()
    };

    // If in a game P1 has the highest card and its value is higher than the total number of cards
    // in the game, P1 wins automatically.
    let highest = *d1.iter().max().unwrap();
    if highest > (d1.len() + d2.len()) as u64 && highest > *d2.iter().max().unwrap() {
        return (true, 0);
    }

    while !d1.is_empty() && !d2.is_empty() {
        if rec && !seen.insert(hash_fn(&d1, &d2)) {
            return (true, score(d1));
        }

        let (a, b) = (d1.pop_front().unwrap(), d2.pop_front().unwrap());

        let p1_wins = if rec && d1.len() >= a as usize && d2.len() >= b as usize {
            let d1_clone = d1.iter().cloned().take(a as usize).collect();
            let d2_clone = d2.iter().cloned().take(b as usize).collect();
            play_game((d1_clone, d2_clone), rec).0
        } else {
            a > b
        };

        if p1_wins {
            d1.push_back(a);
            d1.push_back(b);
        } else {
            d2.push_back(b);
            d2.push_back(a);
        }
    }

    if d1.is_empty() {
        (false, score(d2))
    } else {
        (true, score(d1))
    }
}

fn score(d: VecDeque<u64>) -> u64 {
    d.into_iter().rev().zip(1..).map(|(c, n)| c * n).sum()
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day22.txt"))),
        33434
    },
    p2 => {
        part_2(parse_input(include_str!("../inputs/day22.txt"))),
        31657
    }
}
