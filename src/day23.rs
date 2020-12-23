use std::collections::VecDeque;

use itertools::Itertools;

pub fn parse_input<S: AsRef<[u8]>>(input: S) -> VecDeque<u8> {
    input.as_ref().iter().map(|c| c - b'0').collect()
}

pub fn part_1(mut list: VecDeque<u8>) -> String {
    for _ in 0..100 {
        let head = list.pop_front().unwrap();
        let picked = list.drain(..3).collect_vec();

        let mut dest = dec(head);
        while picked.contains(&dest) {
            dest = dec(dest);
        }

        let (idx, _) = list.iter().find_position(|&&x| x == dest).unwrap();
        for x in picked.into_iter().rev() {
            list.insert(idx + 1, x);
        }

        list.push_back(head);
    }

    let (rot, _) = list.iter().find_position(|&&x| x == 1).unwrap();
    list.rotate_left(rot);
    list.pop_front();

    String::from_utf8(
        list.make_contiguous()
            .iter()
            .map(|c| c + b'0')
            .collect_vec(),
    )
    .unwrap()
}

fn dec(x: u8) -> u8 {
    if x == 1 {
        9
    } else {
        x - 1
    }
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day23.txt"))),
        "25368479"
    }
}
