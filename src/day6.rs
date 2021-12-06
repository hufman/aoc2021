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
pub fn solve_part1(starting_fish: &[Fish]) -> u32 {
    let mut fishes: Vec<Fish> = starting_fish.to_vec();
    for _day in 0..80 {
        cycle(&mut fishes)
    }
    fishes.len() as u32
}

fn cycle(fishes: &mut Vec<Fish>) {
    let mut new_fishes: Vec<Fish> = Vec::new();
    fishes.iter_mut().for_each(|f| {
        if f.age == 0 {
            f.age = 7;
            new_fishes.push(Fish { age: 8 })
        }
        f.age -=1;
    });
    fishes.extend(new_fishes)
}