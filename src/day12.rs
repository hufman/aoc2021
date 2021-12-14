use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::day12::AocGraphNodeType::{END, LARGE, SMALL};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AocGraphNodeType {
    START,
    LARGE,
    SMALL,
    END,
}

#[derive(Clone, Debug)]
pub struct AocGraphNode {
    name: String,
    node_type: AocGraphNodeType,
    visited: bool
}

impl PartialEq<Self> for AocGraphNode {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for AocGraphNode {
}
impl Hash for AocGraphNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl From<&str> for AocGraphNode {
    fn from(input: &str) -> Self {
        if input == "start" {
            AocGraphNode{name: input.to_string(), node_type: AocGraphNodeType::START, visited: false}
        } else if input == "end" {
            AocGraphNode{name: input.to_string(), node_type: AocGraphNodeType::END, visited: false}
        } else if input.chars().all(|c| c.is_lowercase()) {
            AocGraphNode{name: input.to_string(), node_type: AocGraphNodeType::SMALL, visited: false}
        } else if input.chars().all(|c| c.is_uppercase()) {
            AocGraphNode { name: input.to_string(), node_type: AocGraphNodeType::LARGE, visited: false }
        } else {
            panic!("Failed to convert {} to AocGraphNode", input)
        }
    }
}

struct AocGraphSegment {
    left: AocGraphNode,
    right: AocGraphNode
}

pub struct AocGraphPuzzle {
    pub nodes: HashMap<String, AocGraphNode>,
    neighbors: HashMap<String, Vec<String>>,
    spare_time: bool,
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> AocGraphPuzzle {
    let segments: Vec<AocGraphSegment> = input.lines()
        .map(|line| build_graph_segment(line))
        .flatten()
        .collect();
    let mut nodes = HashMap::new();
    segments.iter().for_each(|s| {
        if !nodes.contains_key(&s.left.name) {
            nodes.insert(s.left.name.clone(), s.left.clone());
        }
        if !nodes.contains_key(&s.right.name) {
            nodes.insert(s.right.name.clone(), s.right.clone());
        }
    });
    let neighbors = build_graph_neighbors(&segments);
    AocGraphPuzzle{nodes, neighbors, spare_time:false}
}

fn build_graph_segment(line: &str) -> Option<AocGraphSegment> {
    let splits: Vec<&str> = line.split("-").collect();
    if splits.len() == 2 {
        let left = AocGraphNode::from(splits[0]);
        let right = AocGraphNode::from(splits[1]);
        Some(AocGraphSegment{left, right})
    } else {
        None
    }
}

fn build_graph_neighbors(lines: &[AocGraphSegment]) -> HashMap<String, Vec<String>> {
    let mut result: HashMap<String, Vec<String>> = HashMap::with_capacity(lines.len());
    lines.iter().for_each(|segment: &AocGraphSegment| {
        result.entry(segment.left.name.clone()).or_insert(Vec::new()).push(segment.right.name.clone());
        result.entry(segment.right.name.clone()).or_insert(Vec::new()).push(segment.left.name.clone());
    });
    result
}

#[aoc(day12, part1)]
pub fn solve_part1(puzzle: &AocGraphPuzzle) -> u64 {
    let mut solution = AocGraphPuzzle{nodes: puzzle.nodes.to_owned(), neighbors: puzzle.neighbors.to_owned(), spare_time:false};
    count_routes(&mut solution, &"start".to_string())
}

#[aoc(day12, part2)]
pub fn solve_part2(puzzle: &AocGraphPuzzle) -> u64 {
    let mut solution = AocGraphPuzzle{nodes: puzzle.nodes.to_owned(), neighbors: puzzle.neighbors.to_owned(), spare_time:true};
    count_routes(&mut solution, &"start".to_string())
}

fn count_routes(puzzle: &mut AocGraphPuzzle, starting: &String) -> u64 {
    let mut route_count = 0;
    // we are using up spare time in this room
    let had_spare_time = puzzle.spare_time;
    let bonus_time = puzzle.spare_time && puzzle.nodes[starting].node_type == SMALL && puzzle.nodes[starting].visited;
    puzzle.spare_time = puzzle.spare_time && !bonus_time;
    puzzle.nodes.get_mut(starting).unwrap().visited = true;

    let neighbors = puzzle.neighbors[starting].to_owned();
    neighbors.iter().for_each(|node_name: &String| {
        let node = puzzle.nodes[node_name].to_owned();
        if node.node_type == LARGE ||
            (node.node_type == SMALL && !node.visited) ||
            (node.node_type == SMALL && node.visited && puzzle.spare_time) {
            route_count += count_routes(puzzle, &node.name)
        };
        if node.node_type == END {
            route_count += 1
        }
    });
    // unwind this visit state
    puzzle.nodes.get_mut(starting).unwrap().visited = bonus_time;
    puzzle.spare_time = had_spare_time;
    route_count
}