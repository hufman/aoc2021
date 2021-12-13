use std::fmt::{Debug, Formatter};
use crate::helpers::Point;

pub struct Origami {
    pub grid: Vec<Vec<bool>>,
    pub folds: Vec<Point>,
}
impl Debug for Origami {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..11 {
            let line: Vec<&str> = self.grid[y][0..100]
                .iter()
                .map(|x| if *x {"#"} else {" "})
                .collect();
            f.debug_list().entry(&line).finish().unwrap();
            f.write_str("\n").unwrap();
        }
        f.write_str("")
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Origami {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let points: Vec<Point> = parts[0].lines().map(|line| Point::from(line)).collect();
    let mut grid = Vec::with_capacity(2000);
    for _ in 0..2000 {
        grid.push(vec![false; 2000])
    }
    points.into_iter().for_each(|p| {
        grid[p.y][p.x] = true
    });
    let folds = parts[1].lines().map(|line| parse_folds(line)).collect();
    Origami {grid, folds}
}

fn parse_folds(line: &str) -> Point {
    let splits: Vec<&str> = line.split_whitespace().collect();
    let split: Vec<&str> = splits[2].split("=").collect();
    match split[0] {
        "x" => Point{x: split[1].parse().unwrap(), y: 0},
        "y" => Point{x: 0, y: split[1].parse().unwrap()},
        _ => panic!("Unable to parse {} as a fold", line),
    }
}

#[aoc(day13, part1)]
pub fn solve_part1(puzzle: &Origami) -> u32 {
    let mut solution: Vec<Vec<bool>> = puzzle.grid.iter().map(|row| row.clone()).collect();

    fold(&mut solution, &puzzle.folds[0]);
    count_points(&solution)
}

#[aoc(day13, part2)]
pub fn solve_part2(puzzle: &Origami) -> bool {
    let mut solution: Vec<Vec<bool>> = puzzle.grid.iter().map(|row| row.clone()).collect();
    puzzle.folds.iter().for_each(|f| fold(&mut solution, &f));

    println!("{:?}", Origami{grid: solution, folds:puzzle.folds.iter().map(|a|a.clone()).collect()});
    true
}

pub fn fold(grid: &mut Vec<Vec<bool>>, fold: &Point) {
    if fold.x > 0 {
        let center = fold.x;
        for x in 1..=fold.x {
            for y in 0..2000 {
                if grid[y][center+x] {
                    grid[y][center+x] = false;
                    grid[y][center-x] = true;
                }
            }
        }
    }
    if fold.y > 0 {
        let center = fold.y;
        for y in 1..=fold.y {
            for x in 0..2000 {
                if grid[center+y][x] {
                    println!("Moving x {} y {} to {}", x, center+y, center-y);
                    grid[center+y][x] = false;
                    grid[center-y][x] = true;
                }
            }
        }
    }
}

pub fn count_points(grid: &Vec<Vec<bool>>) -> u32 {
    grid.iter().map(|row| row.iter().filter(|c| **c).count() as u32).sum()
}