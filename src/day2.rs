#[derive(Default)]
pub struct Password {
    a: usize,
    b: usize,
    letter: char,
    text: String,
}

pub fn parse_input<S: AsRef<str>>(input: S) -> Vec<Password> {
    input
        .as_ref()
        .lines()
        .map(|l| {
            let mut pwd = Password::default();
            text_io::scan!(l.bytes() => "{}-{} {}: {}", pwd.a, pwd.b, pwd.letter, pwd.text);
            pwd
        })
        .collect()
}

pub fn part_1(pwds: &[Password]) -> usize {
    pwds.iter()
        .filter(|pwd| {
            let ascii = pwd.text.bytes();
            (pwd.a..=pwd.b).contains(&ascii.filter(|b| *b == pwd.letter as u8).count())
        })
        .count()
}

pub fn part_2(pwds: &[Password]) -> usize {
    pwds.iter()
        .filter(|pwd| {
            let ascii = pwd.text.as_bytes();
            (ascii[pwd.a - 1] == pwd.letter as u8) ^ (ascii[pwd.b - 1] == pwd.letter as u8)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    use lazy_static::lazy_static;

    lazy_static! {
        static ref PASSWORDS: Vec<Password> = parse_input(include_str!("../inputs/day2.txt"));
    }

    #[test]
    fn part_1_solution() {
        assert_eq!(part_1(&PASSWORDS), 548);
    }

    #[test]
    fn part_2_solution() {
        assert_eq!(part_2(&PASSWORDS), 502);
    }
}
