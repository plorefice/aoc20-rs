pub fn parse_input<S: AsRef<str>>(input: S) -> Vec<Vec<bool>> {
    input
        .as_ref()
        .lines()
        .map(|l| l.bytes().map(|b| b == b'#').collect())
        .collect()
}

pub fn part_1(map: &[Vec<bool>]) -> usize {
    check_slope(map, 3, 1)
}

pub fn part_2(map: &[Vec<bool>]) -> usize {
    check_slope(map, 1, 1)
        * check_slope(map, 3, 1)
        * check_slope(map, 5, 1)
        * check_slope(map, 7, 1)
        * check_slope(map, 1, 2)
}

fn check_slope(map: &[Vec<bool>], dx: usize, dy: usize) -> usize {
    let h = map.len();
    let w = map[0].len();

    let xs = (0..).step_by(dx).cycle().map(|x| x % w);
    let ys = (0..h).step_by(dy);

    xs.zip(ys).filter(|(x, y)| map[*y][*x]).count()
}

crate::solutions!(
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day03.txt"))),
        234
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day03.txt"))),
        5813773056
    }
);
