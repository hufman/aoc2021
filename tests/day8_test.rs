use std::collections::HashSet;
use aoc2021::day8::{decode_signal_segments, decode_signals, input_generator};

#[test]
fn day8b_example() {
    let input = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    let scramble = input_generator(input);
    let signals = decode_signal_segments(&scramble[0].signals);
    let a = "a".to_string();
    let b = "b".to_string();
    let c = "c".to_string();
    let d = "d".to_string();
    let e = "e".to_string();
    let f = "f".to_string();
    let g = "g".to_string();

    assert_eq!(HashSet::from_iter([c.clone(), a.clone(), g.clone(), e.clone(), d.clone(), b.clone()]).to_owned(), signals[0]);
    assert_eq!(HashSet::from_iter([a.clone(), b.clone()]).to_owned(), signals[1]);
    assert_eq!(HashSet::from_iter([g.clone(), c.clone(), d.clone(), f.clone(), a.clone()]).to_owned(), signals[2]);
    assert_eq!(HashSet::from_iter([f.clone(), b.clone(), c.clone(), a.clone(), d.clone()]).to_owned(), signals[3]);
    assert_eq!(HashSet::from_iter([e.clone(), a.clone(), f.clone(), b.clone()]).to_owned(), signals[4]);
    assert_eq!(HashSet::from_iter([c.clone(), d.clone(), f.clone(), b.clone(), e.clone()]).to_owned(), signals[5]);
    assert_eq!(HashSet::from_iter([c.clone(), d.clone(), f.clone(), g.clone(), e.clone(), b.clone()]).to_owned(), signals[6]);
    assert_eq!(HashSet::from_iter([d.clone(), a.clone(), b.clone()]).to_owned(), signals[7]);
    assert_eq!(HashSet::from_iter([a.clone(), c.clone(), e.clone(), d.clone(), g.clone(), f.clone(), b.clone()]).to_owned(), signals[8]);
    assert_eq!(HashSet::from_iter([c.clone(), e.clone(), f.clone(), a.clone(), b.clone(), d.clone()]).to_owned(), signals[9]);
}