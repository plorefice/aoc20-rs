use std::collections::HashSet;

type Coord = (i64, i64, i64, i64);

pub fn parse_input<S: AsRef<[u8]>>(input: S) -> HashSet<Coord> {
    let mut set = HashSet::new();

    for (x, line) in input.as_ref().split(|b| *b == b'\n').enumerate() {
        for (y, b) in line.iter().enumerate() {
            if *b == b'#' {
                set.insert((x as i64, y as i64, 0, 0));
            }
        }
    }

    set
}

pub fn part_1(input: HashSet<Coord>) -> usize {
    run(input, neigh3)
}

pub fn part_2(input: HashSet<Coord>) -> usize {
    run(input, neigh4)
}

fn run<F>(mut map: HashSet<Coord>, neighs: F) -> usize
where
    F: Fn(Coord) -> Vec<Coord>,
{
    for _ in 0..6 {
        let mut new = HashSet::with_capacity(map.len());

        for coord in &map {
            let to_check = neighs(*coord);

            for c in to_check {
                let n = neighs(c).iter().filter(|c| map.contains(c)).count();

                if (map.contains(&c) && (n == 2 || n == 3)) || (!map.contains(&c) && n == 3) {
                    new.insert(c);
                }
            }
        }

        map = new;
    }

    map.len()
}

fn neigh3((x, y, z, _): Coord) -> Vec<Coord> {
    let mut vec = Vec::with_capacity(26);

    for i in x - 1..=x + 1 {
        for j in y - 1..=y + 1 {
            for k in z - 1..=z + 1 {
                if (i, j, k) != (x, y, z) {
                    vec.push((i, j, k, 0));
                }
            }
        }
    }

    vec
}

fn neigh4((x, y, z, w): Coord) -> Vec<Coord> {
    let mut vec = Vec::with_capacity(80);

    for i in x - 1..=x + 1 {
        for j in y - 1..=y + 1 {
            for k in z - 1..=z + 1 {
                for h in w - 1..=w + 1 {
                    if (i, j, k, h) != (x, y, z, w) {
                        vec.push((i, j, k, h));
                    }
                }
            }
        }
    }

    vec
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_bytes!("../inputs/day17.txt"))),
        386
    },
    p2 => {
        part_2(parse_input(include_bytes!("../inputs/day17.txt"))),
        2276
    }
}
