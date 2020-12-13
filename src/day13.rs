use std::iter;

pub fn parse_input<S: AsRef<str>>(input: S) -> (i128, Vec<(i128, i128)>) {
    let mut lines = input.as_ref().lines();

    let ts = lines.next().unwrap().parse().unwrap();
    let ids = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, id)| {
            if id == "x" {
                None
            } else {
                Some((i as i128, id.parse().unwrap()))
            }
        })
        .collect();

    (ts, ids)
}

pub fn part_1((ts, ids): &(i128, Vec<(i128, i128)>)) -> i128 {
    let mut times: Vec<_> = ids
        .iter()
        .map(|&(_, id)| {
            iter::successors(Some(id), |t| Some(t.wrapping_add(id)))
                .find_map(|t| if t >= *ts { Some((id, t - ts)) } else { None })
                .unwrap()
        })
        .collect();

    times.sort_by_key(|&(_, t)| t);

    times[0].0 * times[0].1
}

pub fn part_2((_, ids): &(i128, Vec<(i128, i128)>)) -> i128 {
    ids.iter()
        .skip(1)
        .fold((0, ids[0].1), |(r0, x0), &(r2, x1)| {
            let prod = x0 * x1;
            let (b, a) = egcd(x1, x0);
            let r = ((r0 * x1 * b) + ((x1 - r2) * x0 * a)) % prod;
            let r = if r < 0 { r + prod } else { r };

            (r, prod)
        })
        .0
}

fn egcd(a: i128, b: i128) -> (i128, i128) {
    let mut r0 = a;
    let mut r1 = b;
    let mut s0 = 1;
    let mut s1 = 0;
    let mut t0 = 0;
    let mut t1 = 1;

    while r1 != 0 {
        let q = r0 / r1;
        let r2 = r0 - q * r1;
        let s2 = s0 - q * s1;
        let t2 = t0 - q * t1;
        r0 = r1;
        s0 = s1;
        t0 = t1;
        r1 = r2;
        s1 = s2;
        t1 = t2;
    }
    (s0, t0)
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day13.txt"))),
        5946
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day13.txt"))),
        645_338_524_823_718
    }
}
