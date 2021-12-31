use std::collections::HashMap;
use std::ops::{Add, Range};
use std::str::FromStr;

pub struct FlipCuboid {
    direction: bool,
    x: Range<i32>,
    y: Range<i32>,
    z: Range<i32>,
}
pub struct FlipCuboidParseError {}
impl FromStr for FlipCuboid {
    type Err = FlipCuboidParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return Result::Err(FlipCuboidParseError{})
        }
        let direction = match parts[0] {
            "on" => true,
            "off" => false,
            _ => return Result::Err(FlipCuboidParseError{})
        };
        let ranges: Vec<Range<i32>> = parts[1].split(',')
            .map(|field| field.split('=').collect::<Vec<&str>>())
            .filter(|fields| fields.len() == 2)
            .map(|fields| range_from_str(fields[1]))
            .flatten()
            .collect();
        if ranges.len() != 3 {
            return Result::Err(FlipCuboidParseError{})
        }
        let x = ranges[0].clone();
        let y = ranges[1].clone();
        let z = ranges[2].clone();
        Result::Ok(FlipCuboid{direction, x, y, z})
    }
}

fn range_from_str<T>(s: &str) -> Result<Range<T>, FlipCuboidParseError>
    where T: FromStr + Clone + Add<Output = T> + From<u8> {
    let parts: Vec<T> = s.split("..")
        .map(|f| f.parse::<T>())
        .flatten()
        .collect();
    if parts.len() != 2 {
        return Err(FlipCuboidParseError{})
    }
    return Ok(Range{start: parts[0].clone(), end: parts[1].clone() + T::from(1)})
}

struct SparseWorld {
    grid: HashMap<i32, HashMap<i32, HashMap<i32, bool>>>
}
impl SparseWorld {
    fn new() -> Self {
        SparseWorld{grid: HashMap::new()}
    }
    fn set_cuboid(&mut self, instruction: &FlipCuboid) {
        instruction.x.clone().for_each(|x| {
            instruction.y.clone().for_each(|y| {
                instruction.z.clone().for_each(|z| {
                    self.grid.entry(x).or_insert(HashMap::new())
                        .entry(y).or_insert(HashMap::new())
                        .insert(z, instruction.direction);
                })
            })
        })
    }
    fn on_points(&self) -> Vec<(i32, i32, i32, bool)> {
        self.grid.iter()
            .map(|(x, ys)| ys.iter()
                .map(|(y, zs)| zs.iter()
                    .filter(|(_z, v)| **v)
                    .map(|(z, v)| (*x, *y, *z, *v))
                )
                .flatten()
            )
            .flatten()
            .collect()
    }
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<FlipCuboid> {
    input.lines()
        .map(|line| line.parse())
        .flatten()
        .collect()
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &[FlipCuboid]) -> u32 {
    let mut area = SparseWorld::new();
    input.iter()
        .filter(|f| {
            f.x.start >= -50 && f.x.end <= 51 &&
            f.y.start >= -50 && f.y.end <= 51 &&
            f.z.start >= -50 && f.z.end <= 51
        })
        .for_each(|f| {
            area.set_cuboid(f)
        });
    area.on_points()
        .iter()
        .count() as u32
}