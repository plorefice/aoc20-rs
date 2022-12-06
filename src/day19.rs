use std::collections::HashMap;

use itertools::Itertools;
use smallvec::{smallvec, SmallVec};

#[derive(Debug)]
pub enum Rule {
    Term(u8),
    And(SmallVec<[u16; 4]>),
    Or((SmallVec<[u16; 4]>, SmallVec<[u16; 4]>)),
}

pub fn parse_input(input: &str) -> (HashMap<u16, Rule>, Vec<&[u8]>) {
    let mut rule_map = HashMap::new();
    let (rules, inputs) = input.split("\n\n").next_tuple().unwrap();

    let parse_rule = |r: &str| -> SmallVec<[u16; 4]> {
        r.split(' ')
            .map(str::parse)
            .collect::<Result<_, _>>()
            .unwrap()
    };

    for rule in rules.lines() {
        let (id, rule) = rule.split(": ").next_tuple().unwrap();

        let id = id.parse().unwrap();

        if &rule[..1] == "\"" {
            rule_map.insert(id, Rule::Term(rule.as_bytes()[1]));
        } else if rule.contains('|') {
            rule_map.insert(
                id,
                Rule::Or(rule.split(" | ").map(parse_rule).next_tuple().unwrap()),
            );
        } else {
            rule_map.insert(id, Rule::And(parse_rule(rule)));
        }
    }

    (rule_map, inputs.lines().map(|l| l.as_bytes()).collect())
}

pub fn part_1(input: &(HashMap<u16, Rule>, Vec<&[u8]>)) -> usize {
    solve(input, false)
}

pub fn part_2(input: &mut (HashMap<u16, Rule>, Vec<&[u8]>)) -> usize {
    input
        .0
        .insert(8, Rule::Or((smallvec![42], smallvec![42, 8])));

    input
        .0
        .insert(11, Rule::Or((smallvec![42, 31], smallvec![42, 11, 31])));

    solve(input, true)
}

pub fn solve((rules, inputs): &(HashMap<u16, Rule>, Vec<&[u8]>), p2: bool) -> usize {
    inputs
        .iter()
        .filter(|&&input| {
            let mut s = input;

            // I assume rule #0 is [8 11] for every input, since the problem text hints at that.
            // In part 2, keep applying rule 8 until we either hit a match or run out of input.
            while !s.is_empty() {
                let (valid, rest) = validate(s, rules, 8);
                if !valid {
                    break;
                }

                let (valid, last) = validate(rest, rules, 11);
                if valid && last.is_empty() {
                    return true;
                } else if p2 {
                    s = rest;
                    continue;
                } else {
                    break;
                }
            }

            false
        })
        .count()
}

fn validate<'a>(input: &'a [u8], rules: &HashMap<u16, Rule>, id: u16) -> (bool, &'a [u8]) {
    let validate_list = |input: &'a [u8], rs: &SmallVec<[u16; 4]>| -> (bool, &'a [u8]) {
        let mut s = input;

        for r in rs {
            let (ok, rest) = validate(s, rules, *r);
            if !ok {
                return (false, input);
            }
            s = rest;
        }
        (true, s)
    };

    if input.is_empty() {
        return (false, input);
    }

    match &rules[&id] {
        Rule::Term(ch) => (input[0] == *ch, &input[1..]),
        Rule::And(rs) => {
            let (ok, rest) = validate_list(input, rs);
            if !ok {
                (false, input)
            } else {
                (true, rest)
            }
        }
        Rule::Or((lhs, rhs)) => {
            let (ok, rest) = validate_list(input, lhs);
            if ok {
                (true, rest)
            } else {
                let (ok, rest) = validate_list(input, rhs);
                if ok {
                    (true, rest)
                } else {
                    (false, input)
                }
            }
        }
    }
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day19.txt"))),
        122
    },
    p2 => {
        part_2(&mut parse_input(include_str!("../inputs/day19.txt"))),
        287
    }
}
