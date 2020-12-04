use std::collections::HashMap;

const REQUIRED_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

pub fn parse_input(input: &str) -> Vec<HashMap<&str, &str>> {
    input
        .split("\n\n")
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|w| {
                    let mut kv = w.split(':');
                    (kv.next().unwrap(), kv.next().unwrap())
                })
                .collect()
        })
        .collect()
}

pub fn part_1(input: &[HashMap<&str, &str>]) -> usize {
    input
        .iter()
        .filter(|map| REQUIRED_KEYS.iter().all(|k| map.contains_key(k)))
        .count()
}

pub fn part_2(input: &[HashMap<&str, &str>]) -> usize {
    input
        .iter()
        .filter(|map| REQUIRED_KEYS.iter().all(|k| map.contains_key(k)))
        .filter(|map| {
            map.iter().all(|(k, v)| match *k {
                "byr" => {
                    let yr = v.parse::<u32>().unwrap();
                    yr >= 1920 && yr <= 2002
                }
                "iyr" => {
                    let yr = v.parse::<u32>().unwrap();
                    yr >= 2010 && yr <= 2020
                }
                "eyr" => {
                    let yr = v.parse::<u32>().unwrap();
                    yr >= 2020 && yr <= 2030
                }
                "hgt" if v.ends_with("in") => {
                    let h = v.trim_end_matches("in").parse::<u32>().unwrap();
                    h >= 59 && h <= 76
                }
                "hgt" if v.ends_with("cm") => {
                    let h = v.trim_end_matches("cm").parse::<u32>().unwrap();
                    h >= 150 && h <= 193
                }
                "hcl" if v.len() == 7 && v.as_bytes()[0] == b'#' => {
                    v.bytes().skip(1).all(|b| b.is_ascii_hexdigit())
                }
                "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(v),
                "pid" if v.len() == 9 => v.bytes().all(|b| b.is_ascii_digit()),
                "cid" => true,
                _ => false,
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_solution() {
        assert_eq!(
            part_1(&parse_input(include_str!("../inputs/day4.txt"))),
            247
        );
    }

    #[test]
    fn part_2_solution() {
        assert_eq!(
            part_2(&parse_input(include_str!("../inputs/day4.txt"))),
            145
        );
    }
}
