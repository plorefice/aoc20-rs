use std::collections::HashMap;

type Rules<'a> = HashMap<&'a str, Vec<(usize, &'a str)>>;

pub fn parse_input(input: &str) -> Rules {
    let mut rules = HashMap::with_capacity(1024);

    for line in input.lines() {
        let mut halves = line.split(" contain ");

        let outer = halves.next().unwrap().trim_end_matches(" bags");
        let inners = halves.next().unwrap().trim_end_matches('.');

        let inners = if inners == "no other bags" {
            Vec::new()
        } else {
            inners
                .split(", ")
                .map(|inner| {
                    let mut words = inner.splitn(2, ' ');
                    let count = words.next().unwrap().parse::<usize>().unwrap();
                    (
                        count,
                        words
                            .next()
                            .unwrap()
                            .trim_end_matches(" bag")
                            .trim_end_matches(" bags"),
                    )
                })
                .collect::<Vec<_>>()
        };

        rules.insert(outer, inners);
    }

    rules
}

pub fn part_1(rules: &Rules) -> usize {
    rules
        .iter()
        .filter(|(k, _)| contains_shiny_gold(&rules, k))
        .count()
}

pub fn part_2(rules: &Rules) -> usize {
    count_bags(&rules, "shiny gold")
}

fn contains_shiny_gold(rules: &HashMap<&str, Vec<(usize, &str)>>, bag: &str) -> bool {
    rules[bag]
        .iter()
        .any(|(_, bag)| bag == &"shiny gold" || contains_shiny_gold(rules, bag))
}

fn count_bags(rules: &Rules, bag: &str) -> usize {
    rules[bag]
        .iter()
        .fold(0, |total, (n, bag)| total + n + n * count_bags(rules, bag))
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day7.txt"))),
        265
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day7.txt"))),
        14177
    }
}
