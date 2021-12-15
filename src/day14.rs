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
pub fn solve_part1(input: &PolymerInput) -> u64 {
    expand_polymer_count(input, 10)
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &PolymerInput) -> u64 {
    expand_polymer_count(input, 40)
}

fn expand_polymer_count(input: &PolymerInput, rounds: usize) -> u64 {
    let rules = input.rules.iter().map(|entry| {
        let chars: Vec<char> = entry.0.chars().collect();
        (entry.0.to_string(), vec![chars[0].to_string() + entry.1, entry.1.to_owned() + &chars[1].to_string()])
    }).collect::<HashMap<String, Vec<String>>>();
    let mut pairs = count_pairs(&input.seed);
    for _ in 0..rounds {
        pairs = expand_polymer(pairs, &rules);
    }
    score_polymer(&pairs, input.seed.chars().nth(0).unwrap(), input.seed.chars().nth(input.seed.len()-1).unwrap())
}

fn count_pairs(seed: &String) -> HashMap<String, u64> {
    seed.chars()
        .tuple_windows::<(_, _)>()
        .map(|chars| chars.0.to_string() + &chars.1.to_string())
        .fold(HashMap::new(), |mut map, key| {
            *map.entry(key.to_string()).or_insert(0) += 1;
            map
        })
}

pub fn expand_polymer(pairs: HashMap<String, u64>, rules: &HashMap<String, Vec<String>>) -> HashMap<String, u64> {
    pairs.into_iter().fold(HashMap::new(), |mut output, entry| {
        rules[&entry.0].iter().for_each(|child| {
            *output.entry(child.to_string()).or_insert(0) += entry.1
        });
        output
    })
}

fn count_polymer(pairs: &HashMap<String, u64>, first_letter: char, last_letter: char) -> HashMap<char, u64> {
    let mut counts: HashMap<char, u64> = pairs.iter()
        .fold(HashMap::new(), |mut map: HashMap<char, u64>, element| {
            let chars: Vec<char> = element.0.chars().collect();
            *map.entry(chars[0]).or_insert(0) += element.1;
            *map.entry(chars[1]).or_insert(0) += element.1;
            map
        })
        .into_iter()
        .collect();
    // double count the front and the end
    counts.entry(first_letter).and_modify(|f| *f += 1);
    counts.entry(last_letter).and_modify(|f| *f += 1);
    // then divide by two
    counts.into_iter().map(|c| (c.0, c.1/2)).collect()
}
fn score_polymer(pairs: &HashMap<String, u64>, first_letter: char, last_letter: char) -> u64 {
    let counts = count_polymer(pairs, first_letter, last_letter);
    let fields: Vec<(&char, &u64)> = counts.iter().sorted_by(|a, b| a.1.cmp(b.1)).collect();
    fields[fields.len() - 1].1 - fields[0].1
}