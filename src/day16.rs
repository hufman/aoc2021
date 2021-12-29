use std::io::{Cursor, Read};
use bitstream_io::{BigEndian, BitRead, BitReader, Numeric, SignedNumeric};

pub struct BitReaderCounter<R: BitRead> {
    underlying: R,
    bits_read: u64,
}
impl <R: BitRead> BitRead for BitReaderCounter<R> {
    fn read_bit(&mut self) -> std::io::Result<bool> {
        self.bits_read += 1;
        self.underlying.read_bit()
    }
    fn read<U>(&mut self, bits: u32) -> std::io::Result<U> where U: Numeric {
        self.bits_read += bits as u64;
        self.underlying.read(bits)
    }
    fn read_signed<S>(&mut self, bits: u32) -> std::io::Result<S> where S: SignedNumeric {
        self.bits_read += bits as u64;
        self.underlying.read_signed(bits)
    }
    fn skip(&mut self, bits: u32) -> std::io::Result<()> {
        self.bits_read += bits as u64;
        self.underlying.skip(bits)
    }
    fn byte_aligned(&self) -> bool {
        self.underlying.byte_aligned()
    }
    fn byte_align(&mut self) {
        self.underlying.byte_align()
    }
}

pub enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket)
}
impl From<&Packet> for u64 {
    fn from(packet: &Packet) -> Self {
        match packet {
            Packet::Literal(p) => p.value,
            Packet::Operator(p) => p.evaluate()
        }
    }
}

pub struct LiteralPacket {
    pub version: u8,
    pub type_id: u8,
    pub value: u64
}
impl LiteralPacket {
    fn from<R: BitRead>(version: u8, type_id: u8, reader: &mut R) -> Packet {
        let mut value = 0u64;
        let mut data = 0b11111;
        while data & 0b10000 > 0 {
            data = reader.read(5).unwrap();
            value = value << 4 | (data & 0b01111)
        }
        Packet::Literal(LiteralPacket{version, type_id, value})
    }
}

pub struct OperatorPacket {
    pub version: u8,
    pub type_id: u8,
    pub sub_packets: Vec<Packet>
}
impl OperatorPacket {
    fn from<R: BitRead>(version: u8, type_id: u8, reader: &mut BitReaderCounter<R>) -> Packet {
        let mut sub_packets: Vec<Packet> = Vec::new();

        let length_type_id: u8 = reader.read(1).unwrap();
        if length_type_id == 0 {
            let length: u16 = reader.read(15).unwrap();
            let expected_bits_read = reader.bits_read + length as u64;
            while reader.bits_read < expected_bits_read {
                sub_packets.push(Packet::from(reader))
            }
        } else {
            let packet_count: u16 = reader.read(11).unwrap();
            for _ in 0..packet_count {
                sub_packets.push(Packet::from(reader))
            }
        }

        Packet::Operator(OperatorPacket{version, type_id, sub_packets})
    }

    fn evaluate(&self) -> u64 {
        let mut values = self.sub_packets.iter().map(|p| u64::from(p));
        match self.type_id {
            0 => values.sum(),
            1 => values.fold(1, |acc, p| acc*p),
            2 => values.min().unwrap(),
            3 => values.max().unwrap(),
            5 => {let first = values.next().unwrap(); values.fold(first, |left, right| if left > right {1} else {0})},
            6 => {let first = values.next().unwrap(); values.fold(first, |left, right| if left < right {1} else {0})},
            7 => {let first = values.next().unwrap(); values.fold(first, |left, right| if left == right {1} else {0})},
            _ => panic!("Unknown packet type_id {}", self.type_id)
        }
    }
}

fn parse_hex(digit: u8) -> u8 {
    if digit >= 0x30 && digit <= 0x39 {
        digit - 0x30
    } else if digit >= 0x41 && digit <= 0x46 {
        digit - 0x41 + 10
    } else if digit >= 0x61 && digit <= 0x66 {
        digit - 0x61 + 10
    } else {
        panic!("Could not parse hex digit {} 0x{:x?}", digit, digit)
    }
}

struct HexReader<R: Read> {
    hex_data: R
}
impl <R:Read> Read for HexReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut tmp = [0u8; 2];
        let result = self.hex_data.read(&mut tmp);
        if let Result::Ok(count) = result {
            if count >= 2 {
                buf[0] = parse_hex(tmp[0]) << 4 | parse_hex(tmp[1]);
                Result::Ok(1)
            } else {
                Result::Ok(0)
            }
        } else {
            result
        }
    }
}
impl Packet {
    fn from_str(input: &str) -> Self {
        let cursor = Cursor::new(input.as_bytes());
        let decoder = HexReader {hex_data: cursor};
        let reader = BitReader::endian(decoder, BigEndian);
        let mut sub_reader = BitReaderCounter{underlying: reader, bits_read:0};
        Packet::from(&mut sub_reader)
    }
    fn from<R: BitRead>(reader: &mut BitReaderCounter<R>) -> Self {
        let version: u8 = reader.read(3).unwrap();
        let type_id: u8 = reader.read(3).unwrap();
        match type_id {
            4 => LiteralPacket::from(version, type_id, reader),
            _ => OperatorPacket::from(version, type_id, reader)
        }
    }
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Packet {
    Packet::from_str(input)
}

#[aoc(day16, part1)]
pub fn solve_part1(packet: &Packet) -> u64 {
    count_versions(packet)
}

fn count_versions(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal(p) => p.version as u64,
        Packet::Operator(p) => p.version as u64 + p.sub_packets
            .iter().map(|sub_packet| count_versions(sub_packet)).sum::<u64>(),
    }
}

#[aoc(day16, part2)]
pub fn solve_part2(packet: &Packet) -> u64 {
    u64::from(packet)
}