
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
impl From<&str> for Point {
    fn from(line: &str) -> Self {
        let splits: Vec<usize> = line.split(",")
            .map(|p| p.parse::<usize>())
            .flatten()
            .collect();
        if splits.len() != 2 {
            panic!("Failed to parse {} into Point", line);
        } else {
            Point{x: splits[0], y: splits[1]}
        }
    }
}