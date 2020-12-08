use std::collections::HashMap;

use smallvec::SmallVec;

pub struct Rules {
    rules: Vec<SmallVec<[(usize, usize); 16]>>,
    target: usize,
}

impl Rules {
    fn count_containers(&self) -> usize {
        let mut cache = vec![false; self.rules.len()];

        (0..self.rules.len())
            .filter(|&n| Self::contains(&self.rules, n, self.target, &mut cache))
            .count()
    }

    fn count_bags(&self) -> usize {
        self.inner_count(self.target)
    }

    fn inner_count(&self, bag: usize) -> usize {
        self.rules[bag].iter().fold(0, |total, &(n, inner)| {
            total + n + n * self.inner_count(inner)
        })
    }

    fn contains(
        rules: &[SmallVec<[(usize, usize); 16]>],
        who: usize,
        what: usize,
        cache: &mut Vec<bool>,
    ) -> bool {
        if cache[who] {
            return true;
        }

        for &(_, inner) in &rules[who] {
            if inner == what || Self::contains(rules, inner, what, cache) {
                cache[who] = true;
                return true;
            }
        }

        false
    }
}

pub fn parse_input(input: &[u8]) -> Rules {
    let mut rules = vec![SmallVec::<[(usize, usize); 16]>::new(); 1024];
    let mut mapping = HashMap::with_capacity(1024);
    let mut next_id = 0;

    let mut bump_id = || {
        let id = next_id;
        next_id += 1;
        id
    };

    for line in input.split(|b| *b == b'\n') {
        let (target, rest) = skip_whitespaces(line, 2);

        #[allow(clippy::redundant_closure)]
        let target = *mapping.entry(target).or_insert_with(|| bump_id());

        let (_, mut rest) = skip_whitespaces(rest, 2);

        if &rest[..2] != b"no" {
            while !rest.is_empty() {
                let (n, r) = skip_whitespaces(rest, 1);
                let (color, r) = skip_whitespaces(r, 2);
                let (_, r) = skip_whitespaces(r, 1);

                rules[target].push(((n[0] - b'0') as usize, {
                    #[allow(clippy::redundant_closure)]
                    *mapping.entry(color).or_insert_with(|| bump_id())
                }));

                rest = r;
            }
        };
    }

    Rules {
        rules,
        target: mapping[&b"shiny gold"[..]],
    }
}

fn skip_whitespaces(s: &[u8], n: usize) -> (&[u8], &[u8]) {
    let mut seen = 0;

    for (i, b) in s.iter().enumerate() {
        if *b == b' ' {
            seen += 1;
            if seen == n {
                return (&s[..i], &s[i + 1..]);
            }
        }
    }
    (s, b"")
}

pub fn part_1(rules: &Rules) -> usize {
    rules.count_containers()
}

pub fn part_2(rules: &Rules) -> usize {
    rules.count_bags()
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_bytes!("../inputs/day7.txt"))),
        265
    },
    p2 => {
        part_2(&parse_input(include_bytes!("../inputs/day7.txt"))),
        14177
    }
}
