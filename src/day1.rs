extern crate itertools;

use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.lines()
        .map(|line| line.parse::<usize>())
        .filter(|i| i.is_ok())
        .map(|i| i.unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(numbers: &[usize]) -> usize {
    numbers.iter()
        .tuple_windows::<(_, _)>()
        .filter(|(l, r)| l < r)
        .count()
}

#[aoc(day1, part2)]
pub fn solve_part2(numbers: &[usize]) -> usize {
    let sums = numbers.iter()
        .tuple_windows::<(_, _, _)>()
        .map(|(l, m, r)| l + m + r);
    sums.tuple_windows::<(_, _)>()
        .filter(|(l, r)| l < r)
        .count()
}