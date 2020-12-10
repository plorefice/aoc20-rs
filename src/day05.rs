use itertools::Itertools;

pub fn parse_input<S: AsRef<str>>(input: S) -> Vec<usize> {
    input
        .as_ref()
        .lines()
        .map(|w| {
            let row = w
                .bytes()
                .take(7)
                .fold(0, |n, c| (n << 1) + if c == b'B' { 1 } else { 0 });

            let seat = w
                .bytes()
                .skip(7)
                .fold(0, |n, c| (n << 1) + if c == b'R' { 1 } else { 0 });

            row * 8 + seat
        })
        .collect()
}

pub fn part_1(ids: &[usize]) -> usize {
    *ids.iter().max().unwrap()
}

pub fn part_2(mut ids: Vec<usize>) -> usize {
    ids.sort_unstable();

    let mut it = ids.into_iter();
    while let Some((a, b)) = it.next_tuple() {
        if b == a + 2 {
            return a + 1;
        }
    }

    unreachable!()
}

crate::solutions!(
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day05.txt"))),
        913
    },
    p2 => {
        part_2(parse_input(include_str!("../inputs/day05.txt"))),
        717
    }
);
