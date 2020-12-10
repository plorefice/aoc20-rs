use std::str;

const REQUIRED_KEYS: [&[u8]; 7] = [b"byr", b"iyr", b"eyr", b"hgt", b"hcl", b"ecl", b"pid"];

pub fn parse_input(input: &str) -> Vec<Vec<(&[u8], &[u8])>> {
    input
        .split("\n\n")
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|w| {
                    let mut kv = w.split(':');
                    (kv.next().unwrap().as_bytes(), kv.next().unwrap().as_bytes())
                })
                .collect()
        })
        .collect()
}

pub fn part_1(input: &[Vec<(&[u8], &[u8])>]) -> usize {
    input
        .iter()
        .filter(|map| {
            REQUIRED_KEYS
                .iter()
                .all(|key| map.iter().any(|(k, _)| key == k))
        })
        .count()
}

pub fn part_2(input: &[Vec<(&[u8], &[u8])>]) -> usize {
    input
        .iter()
        .filter(|map| {
            REQUIRED_KEYS
                .iter()
                .all(|key| map.iter().any(|(k, _)| key == k))
        })
        .filter(|map| {
            map.iter().all(|(k, v)| match *k {
                b"byr" => {
                    let yr = unsafe { str::from_utf8_unchecked(v) }
                        .parse::<u32>()
                        .unwrap();
                    yr >= 1920 && yr <= 2002
                }
                b"iyr" => {
                    let yr = unsafe { str::from_utf8_unchecked(v) }
                        .parse::<u32>()
                        .unwrap();
                    yr >= 2010 && yr <= 2020
                }
                b"eyr" => {
                    let yr = unsafe { str::from_utf8_unchecked(v) }
                        .parse::<u32>()
                        .unwrap();
                    yr >= 2020 && yr <= 2030
                }
                b"hgt" if v.ends_with(b"in") => {
                    let h = unsafe { str::from_utf8_unchecked(&v[..v.len() - 2]) }
                        .parse::<u32>()
                        .unwrap();
                    h >= 59 && h <= 76
                }
                b"hgt" if v.ends_with(b"cm") => {
                    let h = unsafe { str::from_utf8_unchecked(&v[..v.len() - 2]) }
                        .parse::<u32>()
                        .unwrap();
                    h >= 150 && h <= 193
                }
                b"hcl" if v.len() == 7 && v[0] == b'#' => v[1..].iter().all(u8::is_ascii_hexdigit),
                b"ecl" => [
                    &b"amb"[..],
                    &b"blu"[..],
                    &b"brn"[..],
                    &b"gry"[..],
                    &b"grn"[..],
                    &b"hzl"[..],
                    &b"oth"[..],
                ]
                .contains(v),
                b"pid" if v.len() == 9 => v.iter().all(u8::is_ascii_digit),
                b"cid" => true,
                _ => false,
            })
        })
        .count()
}

crate::solutions!(
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day04.txt"))),
        247
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day04.txt"))),
        145
    }
);
