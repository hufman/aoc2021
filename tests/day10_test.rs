use aoc2021::day10::{check_line, solve_part2};
use aoc2021::day10::ParseResult::Incomplete;

#[test]
fn day10b_example() {
    let lines = vec![
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "(((({<>}<{<{<>}{[]{[]{}",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];
    assert_eq!(Incomplete("}}]])})]".chars().collect()), check_line(lines[0]));
    assert_eq!(Incomplete(")}>]})".chars().collect()), check_line(lines[1]));
    assert_eq!(Incomplete("}}>}>))))".chars().collect()), check_line(lines[2]));
    assert_eq!(Incomplete("]]}}]}]}>".chars().collect()), check_line(lines[3]));
    assert_eq!(Incomplete("])}>".chars().collect()), check_line(lines[4]));

    let lines2: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
    assert_eq!(288957, solve_part2(&lines2))
}