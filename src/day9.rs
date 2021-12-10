use itertools::Itertools;
use crate::day9::BasinState::{MOUNTAIN, UNEXPLORED};

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| {
        line.chars().map(|c| c.to_digit(10))
            .flatten()
            .map(|c| c as u8).collect()
    }).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(lines: &[Vec<u8>]) -> u32 {
    lowest_points(lines).iter().map(|p| lines[p.y][p.x] as u32 + 1).sum()
}

fn lowest_points(lines: &[Vec<u8>]) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            let current = lines[y][x];
            let l = x == 0 || lines[y][x-1] > current;
            let u = y == 0 || lines[y-1][x] > current;
            let r = x == lines[y].len() - 1 || lines[y][x+1] > current;
            let d = y == lines.len() - 1 || lines[y+1][x] > current;
            if l && u && r && d {
                points.push(Point{x, y})
            }
        }
    }
    points
}

#[aoc(day9, part2)]
pub fn solve_part2(lines: &[Vec<u8>]) -> u32 {
    let seeds = lowest_points(lines);
    let sizes: Vec<u32> = seeds.into_iter().map(|p| find_basin_size(lines, p))
        .sorted().rev().collect();
    return sizes[0] * sizes[1] * sizes[2]

}

#[derive(PartialEq)]
enum BasinState {
    MOUNTAIN,
    UNEXPLORED,
    EXPLORED
}
pub fn find_basin_size(lines: &[Vec<u8>], start: Point) -> u32 {
    let mut basin_exploration: Vec<Vec<BasinState>> = Vec::new();

    for y in 0..lines.len() {
        basin_exploration.push(Vec::new());
        for x in 0..lines[y].len() {
            if lines[y][x] == 9 {
                basin_exploration[y].push(MOUNTAIN);
            } else {
                basin_exploration[y].push(UNEXPLORED);
            }
        }
    }

    crawl_basin(&mut basin_exploration, start)
}

fn crawl_basin(map: &mut Vec<Vec<BasinState>>, start: Point) -> u32 {
    let mut distance = 0u32;

    if map[start.y][start.x] == BasinState::UNEXPLORED {
        distance += 1;
        // color this point
        map[start.y][start.x] = BasinState::EXPLORED;

        // left
        if start.x > 0 {
            distance += crawl_basin(map, Point { x: start.x - 1, y: start.y })
        }
        // up
        if start.y > 0 {
            distance += crawl_basin(map, Point { x: start.x, y: start.y - 1 })
        }
        // right
        if start.x < map[start.y].len() - 1 {
            distance += crawl_basin(map, Point { x: start.x + 1, y: start.y })
        }
        // down
        if start.y < map.len() - 1 {
            distance += crawl_basin(map, Point { x: start.x, y: start.y + 1 })
        }
    }
    distance
}