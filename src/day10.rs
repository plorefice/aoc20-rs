use std::collections::HashMap;

pub fn parse_input<S: AsRef<str>>(input: S) -> Vec<usize> {
    let mut v = input
        .as_ref()
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    v.push(0);
    v.sort_unstable();

    v
}

pub fn part_1(adapters: &[usize]) -> usize {
    let mut ones = 0;
    let mut threes = 0;

    for diff in adapters
        .iter()
        .zip(adapters.iter().skip(1))
        .map(|(a, b)| b - a)
    {
        if diff == 1 {
            ones += 1;
        } else if diff == 3 {
            threes += 1;
        }
    }

    ones * (threes + 1)
}

pub fn part_2(adapters: &[usize]) -> usize {
    compute(&mut HashMap::with_capacity(adapters.len()), adapters)
}

fn compute(cache: &mut HashMap<usize, usize>, adapters: &[usize]) -> usize {
    if cache.contains_key(&adapters[0]) {
        return cache[&adapters[0]];
    }

    if adapters.len() == 1 {
        cache.insert(adapters[0], 1);
        return 1;
    }

    let base = adapters[0];
    let mut total = 0;

    for i in 1..adapters.len() {
        if adapters[i] - base > 3 {
            break;
        }

        total += compute(cache, &adapters[i..]);
    }

    cache.insert(base, total);

    total
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day10.txt"))),
        2240
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day10.txt"))),
        99214346656768
    }
}
