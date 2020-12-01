use lazy_static::lazy_static;

lazy_static! {
    static ref VALUES: Vec<u32> = include_str!("../inputs/day1.txt")
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect();
}

pub fn part_1() -> u32 {
    for i in 0..VALUES.len() {
        for j in i + 1..VALUES.len() {
            if VALUES[i] + VALUES[j] == 2020 {
                return VALUES[i] * VALUES[j];
            }
        }
    }

    unreachable!("no answer found")
}

pub fn part_2() -> u32 {
    for i in 0..VALUES.len() {
        for j in i + 1..VALUES.len() {
            for k in j + 1..VALUES.len() {
                if VALUES[i] + VALUES[j] + VALUES[k] == 2020 {
                    return VALUES[i] * VALUES[j] * VALUES[k];
                }
            }
        }
    }

    unreachable!("no answer found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_solution() {
        assert_eq!(part_1(), 1020084);
    }

    #[test]
    fn part_2_solution() {
        assert_eq!(part_2(), 295086480);
    }
}
