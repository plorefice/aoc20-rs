use std::{collections::HashSet, iter::Sum, ops::Add};

use itertools::Itertools;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord(i32, i32, i32);

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sum for Coord {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), Add::add)
    }
}

pub fn parse_input<S: AsRef<[u8]>>(input: S) -> Vec<Vec<Coord>> {
    input
        .as_ref()
        .split(|&c| c == b'\n')
        .map(|line| {
            let mut line = line.iter();
            let mut v = Vec::with_capacity(line.len());

            while let Some(c0) = line.next() {
                v.push(match c0 {
                    b'e' => Coord(1, -1, 0),
                    b'w' => Coord(-1, 1, 0),
                    b's' if *line.next().unwrap() == b'e' => Coord(0, -1, 1),
                    b's' => Coord(-1, 0, 1),
                    b'n' if *line.next().unwrap() == b'e' => Coord(1, 0, -1),
                    b'n' => Coord(0, 1, -1),
                    _ => unreachable!(),
                });
            }

            v
        })
        .collect_vec()
}

pub fn part_1(input: Vec<Vec<Coord>>) -> usize {
    let n = input.len();

    input
        .into_iter()
        .fold(HashSet::<Coord>::with_capacity(n), |mut map, coords| {
            let dest = coords.into_iter().sum();
            if map.contains(&dest) {
                map.remove(&dest);
            } else {
                map.insert(dest);
            }
            map
        })
        .len()
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_bytes!("../inputs/day24.txt"))),
        479
    }
}
