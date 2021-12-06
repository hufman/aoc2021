#[derive(Copy, Clone)]
pub struct Fish {
    age: u8
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Fish> {
    input.split(",")
        .map(|f| u8::from_str_radix(f, 10))
        .flatten()
        .map(|num| Fish{age: num})
        .collect()
}


#[aoc(day6, part1)]
pub fn solve_part1(starting_fish: &[Fish]) -> u64 {
    let mut fish_buckets = [0u64; 9];
    for fish in starting_fish.iter() {
        fish_buckets[fish.age as usize] += 1
    }
    for _day in 0..80 {
        cycle(&mut fish_buckets)
    }
    fish_buckets.iter().sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(starting_fish: &[Fish]) -> u64 {
    let mut fish_buckets = [0u64; 9];
    for fish in starting_fish.iter() {
        fish_buckets[fish.age as usize] += 1
    }
    for _day in 0..256 {
        cycle(&mut fish_buckets)
    }
    fish_buckets.iter().sum()
}

fn cycle(fish_buckets: &mut [u64; 9]) {
    let new_fishes = fish_buckets[0];
    for time_remaining in 0..8 {
        fish_buckets[time_remaining] = fish_buckets[time_remaining + 1]
    }
    fish_buckets[6] += new_fishes;
    fish_buckets[8] = new_fishes
}