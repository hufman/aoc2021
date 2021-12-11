use std::cmp::min;

#[derive(Clone, Debug)]
pub struct Octopus {
    energy_level: u8,
    has_flashed: bool,
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Vec<Octopus>> {
    input.lines().map(|line| {
        line.chars().map(|c| c.to_digit(10))
            .flatten()
            .map(|c| Octopus{energy_level: c as u8, has_flashed: false}).collect()
    }).collect()
}

#[aoc(day11, part1)]
pub fn solve_part1(rows: &[Vec<Octopus>]) -> u32 {
    let mut rows: Vec<Vec<Octopus>> = rows.into_iter()
        .map(|row| row.to_vec())
        .collect();
    let mut count = 0;
    // println!("{:?}", rows);
    for _ in 1..=100 {
        count += timestep(&mut rows);
        // if i == 2 {
        //     println!("{:?}", rows)
        // }
    }
    count
}

#[aoc(day11, part2)]
pub fn solve_part2(rows: &[Vec<Octopus>]) -> u32 {
    let mut rows: Vec<Vec<Octopus>> = rows.into_iter()
        .map(|row| row.to_vec())
        .collect();

    for i in 1..=1000 {
        timestep(&mut rows);
        if rows.iter().all(|r| r.iter().all(|o| o.has_flashed)) {
            return i
        }
    }
    0
}

fn timestep(grid: &mut Vec<Vec<Octopus>>) -> u32 {
    let mut flash_count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            grid[y][x].has_flashed = false;
            grid[y][x].energy_level += 1;
        }
    }

    let mut has_flashed = true;
    while has_flashed {
        has_flashed = false;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x].energy_level >= 10 {
                    has_flashed = true;
                    flash_count += 1;
                    grid[y][x].has_flashed = true;
                    grid[y][x].energy_level = 0;
                    flash_neighbors(grid, x, y)
                }
            }
        }
    }
    flash_count
}

fn flash_neighbors(grid: &mut Vec<Vec<Octopus>>, x: usize, y: usize) {
    let min_y = if y > 0 { y-1 } else { 0 };
    let min_x = if x > 0 { x-1 } else { 0 };
    for ny in min_y ..= min(grid.len()-1, y+1) {
        for nx in min_x ..= min(grid[ny].len()-1, x+1) {
            if !grid[ny][nx].has_flashed {
                // println!("Flashing {}.{} to {}", nx, ny, grid[ny][nx].energy_level+1);
                grid[ny][nx].energy_level += 1
            }
        }
    }
}