pub enum Direction {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}

pub fn parse_input<S: AsRef<str>>(input: S) -> Vec<Direction> {
    input
        .as_ref()
        .lines()
        .map(|line| match line.split_at(1) {
            ("N", val) => Direction::North(val.parse().unwrap()),
            ("S", val) => Direction::South(val.parse().unwrap()),
            ("E", val) => Direction::East(val.parse().unwrap()),
            ("W", val) => Direction::West(val.parse().unwrap()),
            ("L", val) => Direction::Left(val.parse().unwrap()),
            ("R", val) => Direction::Right(val.parse().unwrap()),
            ("F", val) => Direction::Forward(val.parse().unwrap()),
            _ => unreachable!(),
        })
        .collect()
}

pub fn part_1(directions: &[Direction]) -> isize {
    let (mut x, mut y) = (0isize, 0isize);
    let mut facing = 0usize;

    let forward_dirs = [(1, 0), (0, -1), (-1, 0), (0, 1)];

    for dir in directions {
        match dir {
            Direction::North(n) => y += n,
            Direction::South(n) => y -= n,
            Direction::East(n) => x += n,
            Direction::West(n) => x -= n,
            Direction::Left(n) => facing = (facing + 4 - (n / 90) as usize) % 4,
            Direction::Right(n) => facing = (facing + (n / 90) as usize) % 4,
            Direction::Forward(n) => {
                x += n * forward_dirs[facing].0;
                y += n * forward_dirs[facing].1;
            }
        }
    }

    x.abs() + y.abs()
}

pub fn part_2(directions: &[Direction]) -> isize {
    let (mut sx, mut sy) = (0isize, 0isize);
    let (mut wx, mut wy) = (10isize, 1isize);

    let rotate = |wx: isize, wy: isize, n: isize| -> (isize, isize) {
        match n {
            90 => (-wy, wx),
            180 => (-wx, -wy),
            270 => (wy, -wx),
            _ => unreachable!(),
        }
    };

    for dir in directions {
        match dir {
            Direction::North(n) => wy += n,
            Direction::South(n) => wy -= n,
            Direction::East(n) => wx += n,
            Direction::West(n) => wx -= n,
            Direction::Left(n) => {
                let (x, y) = rotate(wx, wy, *n);
                wx = x;
                wy = y;
            }
            Direction::Right(n) => {
                let (x, y) = rotate(wx, wy, 360 - n);
                wx = x;
                wy = y;
            }
            Direction::Forward(n) => {
                sx += n * wx;
                sy += n * wy;
            }
        }
    }

    sx.abs() + sy.abs()
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day12.txt"))),
        441
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day12.txt"))),
        40014
    }
}
