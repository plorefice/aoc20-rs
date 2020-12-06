pub fn part_1(input: &[u8]) -> u32 {
    let mut last_char = b' ';
    let mut total = 0u32;
    let mut current = 0u32;

    for &b in input {
        if b == b'\n' && last_char == b {
            total += current.count_ones();
            current = 0;
            continue;
        }

        if b != b'\n' {
            current |= 1 << (b - b'a');
        }

        last_char = b;
    }

    total + current.count_ones()
}

pub fn part_2(input: &[u8]) -> u32 {
    let mut last_char = b' ';
    let mut total = 0u32;
    let mut person = 0u32;
    let mut group = std::u32::MAX;

    for &b in input {
        if b == b'\n' && last_char == b {
            total += group.count_ones();
            group = std::u32::MAX;
            person = 0;
            continue;
        }

        if b != b'\n' {
            person |= 1 << (b - b'a');
        } else {
            group &= person;
            person = 0;
        }

        last_char = b;
    }

    total + group.count_ones()
}

// pub fn part_2<S: AsRef<str>>(input: S) -> usize {
//     input
//         .as_ref()
//         .split("\n\n")
//         .map(|lines| {
//             lines
//                 .split_ascii_whitespace()
//                 .map(|line| line.bytes().fold(0usize, |ans, b| ans | (1 << (b - b'a'))))
//                 .fold(std::usize::MAX, |all, ans| all & ans)
//                 .count_ones()
//         })
//         .sum::<u32>() as usize
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_solution() {
        assert_eq!(part_1(include_bytes!("../inputs/day6.txt")), 6457);
    }

    #[test]
    fn part_2_solution() {
        assert_eq!(part_2(include_bytes!("../inputs/day6.txt")), 3260);
    }
}
