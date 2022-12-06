use smallvec::SmallVec;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    Free,
    Occupied,
    Floor,
}

#[derive(Debug, Clone)]
pub struct Grid {
    seats: Vec<State>,
    w: isize,
    h: isize,
}

pub fn parse_input<S: AsRef<str>>(input: S) -> Grid {
    let input = input.as_ref();

    let w = input.find('\n').unwrap();
    let h = input.as_bytes().len() / w;

    let seats = input
        .lines()
        .flat_map(|line| {
            line.bytes().map(|c| match c {
                b'L' => State::Free,
                b'.' => State::Floor,
                _ => unreachable!(),
            })
        })
        .collect();

    Grid {
        seats,
        w: w as isize,
        h: h as isize,
    }
}

pub fn part_1(Grid { seats, w, h }: Grid) -> usize {
    let mut current = seats;

    loop {
        let mut new = current.clone();

        for x in 0..w {
            for y in 0..h {
                let state = current[(y * w + x) as usize];
                let new_state = &mut new[(y * w + x) as usize];

                let x = x as isize;
                let y = y as isize;

                let occupied = [
                    (x - 1, y - 1),
                    (x, y - 1),
                    (x + 1, y - 1),
                    (x - 1, y),
                    (x + 1, y),
                    (x - 1, y + 1),
                    (x, y + 1),
                    (x + 1, y + 1),
                ]
                .iter()
                .fold(0, |occupied, &(i, j)| {
                    if i >= 0
                        && i < w
                        && j >= 0
                        && j < h
                        && current[(j * w + i) as usize] == State::Occupied
                    {
                        occupied + 1
                    } else {
                        occupied
                    }
                });

                if state == State::Free && occupied == 0 {
                    *new_state = State::Occupied;
                } else if state == State::Occupied && occupied >= 4 {
                    *new_state = State::Free;
                }
            }
        }

        if new == current {
            return current
                .into_iter()
                .filter(|&s| s == State::Occupied)
                .count();
        } else {
            current = new;
        }
    }
}

pub fn part_2(grid: Grid) -> usize {
    let w = grid.w;
    let h = grid.h;

    let mut current = grid;

    loop {
        let mut new = current.clone();

        for x in 0..w {
            for y in 0..h {
                let state = current.seats[(y * w + x) as usize];
                let new_state = &mut new.seats[(y * w + x) as usize];

                let x = x as isize;
                let y = y as isize;

                let occupied = visible_seats(&current, x, y)
                    .into_iter()
                    .filter(|&seat| seat == State::Occupied)
                    .count();

                if state == State::Free && occupied == 0 {
                    *new_state = State::Occupied;
                } else if state == State::Occupied && occupied >= 5 {
                    *new_state = State::Free;
                }
            }
        }

        if new.seats == current.seats {
            return current
                .seats
                .into_iter()
                .filter(|&s| s == State::Occupied)
                .count();
        } else {
            current = new;
        }
    }
}

fn visible_seats(grid: &Grid, x: isize, y: isize) -> SmallVec<[State; 8]> {
    let mut visible: SmallVec<[State; 8]> = Default::default();

    for dir in &[
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ] {
        let (mut i, mut j) = (x, y);

        loop {
            i += dir.0;
            j += dir.1;

            if i < 0 || i >= grid.w || j < 0 || j >= grid.h {
                break;
            }

            let state = grid.seats[(j * grid.w + i) as usize];
            if state != State::Floor {
                visible.push(state);
                break;
            }
        }
    }

    visible
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day11.txt"))),
        2489
    },
    p2 => {
        part_2(parse_input(include_str!("../inputs/day11.txt"))),
        2180
    }
}
