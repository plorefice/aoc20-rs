use std::collections::HashMap;

use smallvec::SmallVec;

pub fn part_1<S: AsRef<str>>(input: S) -> u64 {
    let mut memory = Vec::with_capacity(64 * 1024);
    let mut and_mask = 0;
    let mut or_mask = 0;

    for line in input.as_ref().lines() {
        let mut sides = line.split(" = ");

        let lhs = sides.next().unwrap();
        let rhs = sides.next().unwrap();

        match lhs {
            "mask" => {
                and_mask = 0;
                or_mask = 0;

                for (i, b) in rhs.bytes().rev().enumerate() {
                    match b {
                        b'0' => and_mask |= 1 << i,
                        b'1' => or_mask |= 1 << i,
                        _ => (),
                    }
                }
            }
            _ => {
                let addr: usize = lhs[4..lhs.len() - 1].parse().unwrap();
                if addr >= memory.len() {
                    memory.resize_with(addr + 1, Default::default);
                }
                memory[addr] = (rhs.parse::<u64>().unwrap() | or_mask) & !and_mask;
            }
        }
    }

    memory.iter().sum()
}

pub fn part_2<S: AsRef<str>>(input: S) -> u64 {
    let mut memory = HashMap::with_capacity(64 * 1024);
    let mut mask = 0_usize;
    let mut undef = 0_usize;
    let mut floating = Vec::new();

    for line in input.as_ref().lines() {
        let mut sides = line.split(" = ");

        let lhs = sides.next().unwrap();
        let rhs = sides.next().unwrap();

        match lhs {
            "mask" => {
                mask = 0;
                undef = 0;

                for (i, b) in rhs.bytes().rev().enumerate() {
                    match b {
                        b'1' => mask |= 1 << i,
                        b'X' => undef |= 1 << i,
                        _ => (),
                    }
                }

                floating = compute_masks(undef);
            }
            _ => {
                let base = (lhs[4..lhs.len() - 1].parse::<usize>().unwrap() | mask) & !undef;
                let val: u64 = rhs.parse().unwrap();

                for (and, or) in &floating {
                    memory
                        .entry((base | or) & !and)
                        .and_modify(|v| *v = val)
                        .or_insert(val);
                }
            }
        }
    }

    memory.values().sum()
}

fn compute_masks(undef: usize) -> Vec<(usize, usize)> {
    let n = 2_usize.pow(undef.count_ones());
    let mut addrs = Vec::with_capacity(n);

    let mut indices = SmallVec::<[usize; 64]>::new();
    for i in 0..64 {
        if (undef & (1 << i)) != 0 {
            indices.push(i);
        }
    }

    for v in 0..n {
        let (mut and, mut or) = (0, 0);

        for (i, idx) in indices.iter().enumerate() {
            if (v >> i) & 1 == 0 {
                and |= 1 << idx;
            } else {
                or |= 1 << idx;
            }
        }

        addrs.push((and, or));
    }

    addrs
}

crate::solutions! {
    p1 => {
        part_1(include_str!("../inputs/day14.txt")),
        11884151942312
    },
    p2 => {
        part_2(include_str!("../inputs/day14.txt")),
        2625449018811
    }
}
