#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|line| line.chars()
            .map(|c|c.to_digit(10))
            .flatten()
            .map(|i| i as u8)
            .collect())
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(data: &[Vec<u8>]) -> u32 {
    let gamma_str: Vec<u8> = data.first()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(i, _)| if count_bits(data, i) < data.len()/2 { 0 } else { 1 })
        .collect();
    let gamma =  to_u32(gamma_str.as_slice());
    let epsilon_str: Vec<u8> = gamma_str.iter()
        .map(|c| if *c == 0 { 1 } else { 0 })
        .collect();
    let epsilon = to_u32(epsilon_str.as_slice());
    gamma * epsilon
}

fn count_bits(data: &[Vec<u8>], bit: usize) -> usize {
    data.iter()
        .filter(|l| l[bit] == 1)
        .count()
}

// https://www.reddit.com/r/rust/comments/36ixl0/comment/crehkpw/
fn to_u32(slice: &[u8]) -> u32 {
    slice.iter().fold(0, |acc, &b| acc*2 + b as u32)
}