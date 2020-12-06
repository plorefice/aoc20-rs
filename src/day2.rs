use std::str;

#[derive(Default)]
pub struct Password<'a> {
    a: usize,
    b: usize,
    letter: u8,
    text: &'a [u8],
}

pub fn parse_input<'a>(input: &'a [u8]) -> Vec<Password<'a>> {
    input
        .split(|b| *b == b'\n')
        .map(|l| {
            let mut words = l.split(u8::is_ascii_whitespace);
            let mut bounds = words.next().unwrap().split(|b| *b == b'-');

            Password {
                a: unsafe { str::from_utf8_unchecked(bounds.next().unwrap()) }
                    .parse()
                    .unwrap(),
                b: unsafe { str::from_utf8_unchecked(bounds.next().unwrap()) }
                    .parse()
                    .unwrap(),
                letter: words.next().unwrap()[0],
                text: words.next().unwrap(),
            }
        })
        .collect()
}

pub fn part_1(pwds: &[Password]) -> usize {
    pwds.iter()
        .filter(|pwd| (pwd.a..=pwd.b).contains(&bytecount::count(pwd.text, pwd.letter)))
        .count()
}

pub fn part_2(pwds: &[Password]) -> usize {
    pwds.iter()
        .filter(|pwd| {
            (pwd.text[pwd.a - 1] == pwd.letter as u8) ^ (pwd.text[pwd.b - 1] == pwd.letter)
        })
        .count()
}

#[cfg(test)]
mod solutions {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(
            part_1(&parse_input(include_bytes!("../inputs/day2.txt"))),
            548
        );
    }

    #[test]
    fn p2() {
        assert_eq!(
            part_2(&parse_input(include_bytes!("../inputs/day2.txt"))),
            502
        );
    }
}
