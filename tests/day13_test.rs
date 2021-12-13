use aoc2021::day13::{count_points, fold, input_generator, Origami, solve_part1};

#[test]
fn day13a_example() {
    let input="6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    let puzzle = input_generator(input);
    assert_eq!(2, puzzle.folds.len());
    assert_eq!(18, count_points(&puzzle.grid));
    assert_eq!(17, solve_part1(&puzzle));

    let mut solution: Vec<Vec<bool>> = puzzle.grid.iter().map(|row| row.clone()).collect();
    fold(&mut solution, &puzzle.folds[0]);
    assert_eq!(17, count_points(&solution));
    fold(&mut solution, &puzzle.folds[1]);
    assert_eq!(16, count_points(&solution));
    println!("{:?}", Origami{grid: solution, folds:puzzle.folds});
}