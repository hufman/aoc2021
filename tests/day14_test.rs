use aoc2021::day14::{expand_polymer, input_generator, solve_part1};

#[test]
fn day14a_example() {
    let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    let parsed = input_generator(input);

    assert_eq!("NNCB", &parsed.seed);
    assert_eq!(1588, solve_part1(&parsed));
}