use std::cmp::{max, min};
use regex::Regex;

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
impl From<&str> for Point {
    fn from(line: &str) -> Self {
        let pieces: Vec<i32> = line.split(",")
            .map(|c| i32::from_str_radix(c, 10).unwrap())
            .collect();
        Point{x: pieces[0] as usize, y: pieces[1] as usize}
    }
}
#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Line> {
    let re = Regex::new(r"(?P<start>\d+,\d+) -> (?P<end>\d+,\d+)").unwrap();
    input.lines()
        .map(|line|re.captures(line))
        .flatten()
        .map(|matches| Line{start: Point::from(&matches["start"]), end: Point::from(&matches["end"])})
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(lines: &[Line]) -> u32 {
    let mut area: [[u8; 1000]; 1000] = [[0u8; 1000]; 1000];
    for line in lines {
        trace_straight_line(&mut area, line)
    }
    // println!("{:?}", area);
    area.into_iter()
        .map(|row| row.into_iter().filter(|c| *c>1u8).count())
        .sum::<usize>() as u32
}

#[aoc(day5, part2)]
pub fn solve_part2(lines: &[Line]) -> u32 {
    let mut area: [[u8; 1000]; 1000] = [[0u8; 1000]; 1000];
    for line in lines {
        trace_straight_line(&mut area, line);
        trace_diagonal_line(&mut area, line)
    }
    // println!("{:?}", area);
    area.into_iter()
        .map(|row| row.into_iter().filter(|c| *c>1u8).count())
        .sum::<usize>() as u32
}

fn trace_straight_line(area: &mut [[u8; 1000]; 1000], line: &Line) {
    if line.start.x == line.end.x {
        for y in min(line.start.y, line.end.y)..=max(line.start.y, line.end.y) {
            area[y][line.start.x] += 1;
        }
    }
    if line.start.y == line.end.y {
        for x in min(line.start.x, line.end.x)..=max(line.start.x, line.end.x) {
            area[line.start.y][x] += 1;
        }
    }
}

fn trace_diagonal_line(area: &mut [[u8; 1000]; 1000], line: &Line) {
    if ((line.start.x-line.end.x) as i32).abs() == ((line.start.y-line.end.y) as i32).abs() {
        let mut y = if line.start.x < line.end.x {
            line.start.y
        } else {
            line.end.y
        };
        let dir: i8 = if line.start.x < line.end.x {
            if line.start.y < line.end.y {1} else {-1}
        } else {
            if line.end.y < line.start.y {1} else {-1}
        };
        for x in min(line.start.x, line.end.x)..=max(line.start.x, line.end.x) {
            area[y][x] += 1;
            if dir > 0 {
                y += 1
            } else {
                y -= 1
            }
        }
    }
}