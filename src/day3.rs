pub fn test_generator() -> Vec<Vec<u8>> {
    vec![
        vec![0, 0, 1, 0, 0],
        vec![1, 1, 1, 1, 0],
        vec![1, 0, 1, 1, 0],
        vec![1, 0, 1, 1, 1],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 1, 1, 1],
        vec![0, 0, 1, 1, 1],
        vec![1, 1, 1, 0, 0],
        vec![1, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 1],
        vec![0, 0, 0, 1, 0],
        vec![0, 1, 0, 1, 0],
    ]
}

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
    let len = data.first().unwrap().len();
    let gamma_str: Vec<u8> = (0..len)
        .map(|i| most_common_bit(data, i))
        .collect();
    let gamma = to_u32(gamma_str.as_slice());
    let epsilon_str: Vec<u8> = gamma_str.iter()
        .map(|c| if *c == 0 { 1 } else { 0 })
        .collect();
    let epsilon = to_u32(epsilon_str.as_slice());
    gamma * epsilon
}

fn count_bits(data: &[Vec<u8>], pos: usize) -> usize {
    data.iter()
        .filter(|l| l[pos] == 1)
        .count()
}

/**
Returns a 1 if the given column has more or the same count of 1 than 0
else 0
*/
fn most_common_bit(data: &[Vec<u8>], pos: usize) -> u8 {
    let threshold = data.len() as f32 / 2.0;
    if (count_bits(data, pos) as f32) < threshold { 0 } else { 1 }
}
/**
Returns a 1 if the given column has lower count of 1 than 0
else 0
 */
fn least_common_bit(data: &[Vec<u8>], pos: usize) -> u8 {
    let threshold = data.len() as f32 / 2.0;
    if (count_bits(data, pos) as f32) < threshold { 1 } else { 0 }
}

// https://www.reddit.com/r/rust/comments/36ixl0/comment/crehkpw/
fn to_u32(slice: &[u8]) -> u32 {
    slice.iter().fold(0, |acc, &b| acc*2 + b as u32)
}


#[aoc(day3, part2)]
pub fn solve_part2(data: &[Vec<u8>]) -> u32 {
    let oxygen = to_u32(filter(data, 0, true).as_slice());
    let co2 = to_u32(filter(data, 0, false).as_slice());
    oxygen * co2
}

fn filter(data: &[Vec<u8>], pos: usize, find_most_common: bool) -> Vec<u8> {
    let criteria = if find_most_common {
        most_common_bit(data, pos)
    } else {
        least_common_bit(data, pos)
    };
    let filtered: Vec<Vec<u8>> = data.iter()
        .filter(|l| l[pos] == criteria)
        .map(|l| l.to_owned())
        .collect();
    // println!("Step {} has {:?}", pos, filtered);
    if filtered.len() == 1 {
        filtered[0].clone()
    } else {
        filter(filtered.as_slice(), pos+1, find_most_common)
    }
}

#[test]
fn test_filter() {
    let oxygen_str = filter(test_generator().as_slice(), 0, true);
    assert_eq!(vec![1,0,1,1,1], oxygen_str)
}