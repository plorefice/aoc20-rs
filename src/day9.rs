use std::str::FromStr;

pub fn parse_input<S: AsRef<str>>(input: S) -> Vec<u64> {
    input
        .as_ref()
        .lines()
        .map(u64::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

pub fn part_1(data: &[u64]) -> u64 {
    'outer: for data in data.windows(26) {
        let (num, preamble) = data.split_last().unwrap();

        for i in 0..preamble.len() {
            for j in i + 1..preamble.len() {
                if preamble[i] != preamble[j] && preamble[i] + preamble[j] == *num {
                    continue 'outer;
                }
            }
        }

        return *num;
    }

    unreachable!()
}

pub fn part_2(data: &[u64], target: u64) -> u64 {
    for i in 0..data.len() {
        let mut min = std::u64::MAX;
        let mut max = 0;
        let mut sum = 0;

        for &datum in &data[i..] {
            sum += datum;
            min = datum.min(min);
            max = datum.max(max);

            if sum == target {
                return min + max;
            }
            if sum > target {
                break;
            }
        }
    }

    unreachable!()
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day9.txt"))),
        88311122
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day9.txt")), 88311122),
        13549369
    }
}
