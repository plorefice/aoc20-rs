use std::collections::HashMap;

pub fn parse_input<S: AsRef<str>>(input: S) -> (usize, HashMap<usize, usize>) {
    let mut input = input.as_ref().rsplit(',');

    (
        input.next().unwrap().parse().unwrap(),
        input
            .rev()
            .enumerate()
            .map(|(i, n)| (n.parse().unwrap(), i))
            .collect(),
    )
}

pub fn part_1(input: (usize, HashMap<usize, usize>)) -> usize {
    solve(input, 2020)
}

pub fn part_2(input: (usize, HashMap<usize, usize>)) -> usize {
    solve(input, 30000000)
}

fn solve((mut next, mut spoken): (usize, HashMap<usize, usize>), run_to: usize) -> usize {
    for turn in spoken.len()..run_to - 1 {
        next = match spoken.insert(next, turn) {
            None => 0,
            Some(t) => turn - t,
        };
    }

    next
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day15.txt"))),
        1618
    },
    p2 => {
        part_2(parse_input(include_str!("../inputs/day15.txt"))),
        548531
    }
}
