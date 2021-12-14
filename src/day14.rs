use std::collections::HashMap;
use itertools::Itertools;

pub struct PolymerInput {
    pub seed: String,
    pub rules: HashMap<String, String>,
}
#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> PolymerInput {
    let mut lines = input.lines();
    let seed = lines.next().unwrap().to_string();
    let rules = lines
        .map(|l| {
            let splits: Vec<&str> = l.split_whitespace().collect();
            if splits.len() == 3 {
                Some((splits[0], splits[2]))
            } else {
                None
            }
        })
        .flatten()
        .fold(HashMap::new(), |mut a: HashMap<String, String>, rule| {a.insert(rule.0.to_string(), rule.1.to_string()); a});
    PolymerInput{seed, rules}
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &PolymerInput) -> u32 {
    let mut expanded = input.seed.to_string();
    for _ in 0..10 {
        expanded = expand_polymer(&expanded, &input.rules);
    }
    score_polymer(&expanded)
}

pub fn expand_polymer(seed: &String, rules: &HashMap<String, String>) -> String {
    seed.chars()
        .tuple_windows::<(_, _)>()
        .enumerate()
        .map(|(index, chars)| {
            let key: String = vec![chars.0, chars.1].into_iter().collect();
            let inserted = &rules[&key];
            if index == 0 {
                vec![chars.0.to_string(), inserted.to_owned(), chars.1.to_string()].into_iter().collect::<Vec<String>>()
            } else {
                vec![inserted.to_owned(), chars.1.to_string()].into_iter().collect::<Vec<String>>()
            }

        })
        .flatten()
        .collect()
}

fn score_polymer(polymer: &String) -> u32 {
    let counts = polymer.chars()
        .fold(HashMap::new(), |mut map: HashMap<char, u32>, element| {
            *map.entry(element).or_insert(0) += 1;
            map
        });
    let fields: Vec<(&char, &u32)> = counts.iter().sorted_by(|a, b| a.1.cmp(b.1)).collect();
    fields[fields.len() - 1].1 - fields[0].1
}