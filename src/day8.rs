use std::collections::HashSet;

#[derive(Debug)]
pub struct Scramble {
    pub signals: Vec<String>,
    pub digits: Vec<String>,
}

type Signal = HashSet<String>;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Scramble> {

    input.lines()
        .map(|l| {
            let fields: Vec<&str> = l.split("|").collect();
            let signals = fields[0].split_whitespace().map(String::from).collect();
            let digits = fields[1].split_whitespace().map(String::from).collect();
            Scramble{signals, digits}
        })
        .collect()
}


#[aoc(day8, part1)]
pub fn solve_part1(scrambles: &[Scramble]) -> u32 {
    scrambles.into_iter()
        .map(|s| s.digits.iter()
            .filter(|d| is_part1_digit(d))
            .count() as u32)
        .sum()
}

fn is_part1_digit(digit: &str) -> bool {
    let size = digit.len();
    size == 2 || size == 3 || size == 4 || size == 7
}

#[aoc(day8, part2)]
pub fn solve_part2(scrambles: &[Scramble]) -> u32 {
    scrambles.into_iter()
        .map(|s| read_display(&decode_signal_segments(&s.signals), &s.digits))
        .sum()
}


pub fn decode_signal_segments(digits: &Vec<String>) -> [Signal; 10] {
    let digit_sets: Vec<Signal> = digits.iter().map(|d| {
        HashSet::from_iter(d.chars().map(|c| c.to_string()).collect::<Vec<String>>())
    }).collect();

    let d1 = signal_segment_where(&digit_sets, |d| d.len() == 2);
    let d3 = signal_segment_where(&digit_sets, |d|
        d.len() == 5 && d.is_superset(&d1));
    let d4 = signal_segment_where(&digit_sets, |d| d.len() == 4);
    let d6 = signal_segment_where(&digit_sets, |d|
        d.len() == 6 && !d.is_superset(&d1)
    );
    let d7 = signal_segment_where(&digit_sets, |d| d.len() == 3);
    let d8 = signal_segment_where(&digit_sets, |d| d.len() == 7);
    let d9 = signal_segment_where(&digit_sets, |d|
        d.len() == 6 && d.is_superset(&d1) && d.is_superset(&d4)
    );
    let d0 = signal_segment_where(&digit_sets, |d|
        d.len() == 6 && d.is_superset(&d1) && !d.is_superset(&d4)
    );
    let d2 = signal_segment_where(&digit_sets, |d|
        d.len() == 5 && !d.is_superset(&d1) && !d.is_subset(&d6)
    );
    let d5 = signal_segment_where(&digit_sets, |d|
        d.len() == 5 && !d.is_superset(&d1) && d.is_subset(&d6)
    );
    [d0, d1, d2, d3, d4, d5, d6, d7, d8, d9]
}

fn signal_segment_where<F>(digit_sets: &Vec<Signal>, f: F) -> Signal
    where F: Fn(&&Signal) -> bool {
    let found: Vec<&Signal> = digit_sets.iter().filter(f).collect();
    assert_eq!(1, found.len());
    found.into_iter().nth(0).unwrap().clone()
}

fn read_display(patterns: &[Signal; 10], digits: &Vec<String>) -> u32 {
    digits.iter()
        .map(|d| find_digit(patterns, d))
        .fold(0, |a, i| a * 10 + i)
}

fn find_digit(patterns: &[Signal; 10], digit: &String) -> u32 {
    let digit_set: Signal = HashSet::from_iter(digit.chars().map(|c| c.to_string()).collect::<Vec<String>>());
    patterns.iter().position(|d| *d == digit_set).unwrap() as u32
}