use std::collections::{BTreeMap, HashMap, HashSet};

use itertools::Itertools;

pub fn parse_input(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    input
        .lines()
        .fold(Vec::with_capacity(64), |mut foods, food| {
            let (ingredients, allergens) = food.split(" (contains ").next_tuple().unwrap();

            let ingredients = ingredients.split_ascii_whitespace().collect();
            let allergens = allergens.trim_end_matches(')').split(", ").collect();

            foods.push((ingredients, allergens));
            foods
        })
}

pub fn part_1(
    input: &[(Vec<&str>, Vec<&str>)],
    possible_allergens: &HashMap<&str, Vec<&str>>,
) -> usize {
    input
        .iter()
        .flat_map(|(ingredients, _)| {
            ingredients
                .iter()
                .filter(|ingr| !possible_allergens.contains_key(*ingr))
        })
        .count()
}

pub fn part_2(mut possible_allergens: HashMap<&str, Vec<&str>>) -> String {
    let mut results: BTreeMap<&str, &str> = BTreeMap::new();

    while !possible_allergens.is_empty() {
        possible_allergens.retain(|k, v| {
            if v.len() == 1 {
                results.insert(v[0], k);
                false
            } else {
                true
            }
        });

        for v in possible_allergens.values_mut() {
            for pinned in results.keys() {
                v.retain(|val| val != pinned);
            }
        }
    }

    results.values().join(",")
}

pub fn solve<'a>(input: &'a [(Vec<&str>, Vec<&str>)]) -> HashMap<&'a str, Vec<&'a str>> {
    let allergens = input.iter().flat_map(|(_, a)| a).unique().collect_vec();

    let mut possible_allergens: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut seen = HashSet::new();

    for (i, (ingredients, _)) in input.iter().enumerate() {
        for ingredient in ingredients {
            if !seen.insert(ingredient) {
                continue;
            }

            'outer: for allergen in &allergens {
                for j in (0..input.len()).filter(|&j| j != i) {
                    if input[j].1.contains(allergen) && !input[j].0.contains(ingredient) {
                        continue 'outer;
                    }
                }

                possible_allergens
                    .entry(ingredient)
                    .or_default()
                    .push(allergen);
            }
        }
    }

    possible_allergens
}

crate::solutions! {
    p1 => {
        {
            let input = parse_input(include_str!("../inputs/day21.txt"));
            part_1(&input, &solve(&input))
        },
        2412
    },
    p2 => {
        part_2(solve(&parse_input(include_str!("../inputs/day21.txt")))),
        "mfp,mgvfmvp,nhdjth,hcdchl,dvkbjh,dcvrf,bcjz,mhnrqp"
    }
}
