use aoc2021::day9::{find_basin_size, input_generator, Point, solve_part1, solve_part2};

#[test]
fn day9a_example() {
    let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
    let data = input_generator(input);
    assert_eq!(15, solve_part1(data.as_slice()));
}

#[test]
fn day9b_example() {
    let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
    let data = input_generator(input);
    assert_eq!(3, find_basin_size(&data, Point{x: 0, y: 0}));
    assert_eq!(9, find_basin_size(&data, Point{x: 9, y: 0}));
    assert_eq!(14, find_basin_size(&data, Point{x: 2, y: 2}));
    assert_eq!(9, find_basin_size(&data, Point{x: 8, y: 4}));
    assert_eq!(1134, solve_part2(&data));
}
