use std::collections::{HashMap, HashSet};

pub fn parse_input<S: AsRef<str>>(input: S) -> Vec<usize> {
    input
        .as_ref()
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect()
}

pub fn part_1(input: &[usize]) -> usize {
    let mut seen = HashSet::with_capacity(input.len());

    for v in input {
        let i = 2020 - v;
        if seen.contains(&i) {
            return v * i;
        }
        seen.insert(v);
    }

    unreachable!("no answer found")
}

/// Part 2 runs in basically O(n*log(n)) due to the input distribution
/// being heavily skewed towards higher numbers.
pub fn part_2(mut input: Vec<usize>) -> usize {
    input.sort_unstable();

    let pivot = input.iter().position(|x| *x > 1010).unwrap();

    let mut table = HashMap::with_capacity(pivot * pivot);
    for i in 0..=pivot {
        for j in (i + 1)..=pivot {
            table.insert(input[i] + input[j], input[i] * input[j]);
        }
    }

    for v in input.iter().rev() {
        if let Some(prod) = table.get(&(2020 - *v)) {
            return v * *prod;
        }
    }

    unreachable!("no answer found")
}

#[cfg(test)]
mod tests {
    use super::*;

    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT: Vec<usize> = parse_input(include_str!("../inputs/day1.txt"));
    }

    #[test]
    fn part_1_solution() {
        assert_eq!(part_1(&INPUT), 1020084);
    }

    #[test]
    fn part_2_solution() {
        assert_eq!(part_2(INPUT.clone()), 295086480);
    }
}
