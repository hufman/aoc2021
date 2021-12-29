use aoc2021::day16::input_generator;
use aoc2021::day16::Packet;

#[test]
fn day16a_example_1() {
    let packet = input_generator("D2FE28");
    assert!(matches!(&packet, Packet::Literal(p)));
    if let Packet::Literal(p) = packet {
        assert_eq!(p.version, 6);
        assert_eq!(p.type_id, 4);
        assert_eq!(p.value, 2021);
    }
}