#![deny(clippy::all)]
#![feature(test)]
extern crate test;

use std::{fs::read_to_string, path::Path};

pub type Input = Packet;

pub const PART_1: usize = 877;
pub const PART_2: usize = 194435634456;

pub fn read_data(data_dir: &str) -> Input {
    let contents = read_to_string(Path::new(data_dir).join("day_16.txt")).unwrap();

    parse_contents(contents.trim())
}

fn parse_contents(contents: &str) -> Input {
    Input::from(contents)
}

pub fn part_1(input: &Input) -> usize {
    input.version_sum()
}

pub fn part_2(input: &Input) -> usize {
    input.evaluate()
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Packet {
    Empty,
    Operator(OperatorPacket),
    Literal(LiteralPacket),
}

impl Default for Packet {
    fn default() -> Self {
        Self::Empty
    }
}

impl Packet {
    fn version_sum(&self) -> usize {
        match self {
            Packet::Operator(p) => p.version_sum(),
            Packet::Literal(p) => p.version_sum(),
            Packet::Empty => 0,
        }
    }

    fn evaluate(&self) -> usize {
        match self {
            Packet::Operator(p) => p.evaluate(),
            Packet::Literal(p) => p.evaluate(),
            Packet::Empty => 0,
        }
    }
}

impl From<&str> for Packet {
    fn from(hex_str: &str) -> Self {
        Biterator::new(hex_str)
            .parse_packet()
            .unwrap_or(Packet::Empty)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
pub struct LiteralPacket {
    version: usize,
    value: usize,
}

impl LiteralPacket {
    fn version_sum(&self) -> usize {
        self.version
    }

    fn evaluate(&self) -> usize {
        self.value
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct OperatorPacket {
    version: usize,
    operator: Operation,
    sub_packets: Vec<Packet>,
}

impl TryFrom<u8> for Operation {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Operation::Sum),
            1 => Ok(Operation::Product),
            2 => Ok(Operation::Minimum),
            3 => Ok(Operation::Maximum),
            5 => Ok(Operation::GreaterThan),
            6 => Ok(Operation::LessThan),
            7 => Ok(Operation::EqualTo),
            _ => Err(format!("invalid operator: {}", value)),
        }
    }
}

impl OperatorPacket {
    fn version_sum(&self) -> usize {
        self.version
            + self
                .sub_packets
                .iter()
                .map(|p| p.version_sum())
                .sum::<usize>()
    }

    fn evaluate(&self) -> usize {
        match self.operator {
            Operation::Sum => self.sub_packets.iter().map(|p| p.evaluate()).sum(),
            Operation::Product => self.sub_packets.iter().map(|p| p.evaluate()).product(),
            Operation::Minimum => self.sub_packets.iter().map(|p| p.evaluate()).min().unwrap(),
            Operation::Maximum => self.sub_packets.iter().map(|p| p.evaluate()).max().unwrap(),
            Operation::GreaterThan => {
                (self.sub_packets[0].evaluate() > self.sub_packets[1].evaluate()) as usize
            }
            Operation::LessThan => {
                (self.sub_packets[0].evaluate() < self.sub_packets[1].evaluate()) as usize
            }
            Operation::EqualTo => {
                (self.sub_packets[0].evaluate() == self.sub_packets[1].evaluate()) as usize
            }
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Biterator {
    contents: Vec<u8>,
    bit_idx: u8,
    byte_idx: usize,
}

impl Biterator {
    fn new(hex_str: &str) -> Self {
        let contents = hex_str
            .as_bytes()
            .chunks(2)
            .map(|c| u8::from_str_radix(&String::from_utf8_lossy(c), 16).unwrap())
            .collect::<Vec<u8>>();

        Self {
            contents,
            bit_idx: 7,
            byte_idx: 0,
        }
    }

    fn parse_packet(&mut self) -> Option<Packet> {
        let version = self.next_n(3)?;
        let type_id = self.next_n(3)? as u8;

        if type_id == 4 {
            let mut out = 0;
            while self.next()? {
                out <<= 4;
                out |= self.next_n(4)?;
            }
            return Some(Packet::Literal(LiteralPacket {
                version,
                value: out << 4 | self.next_n(4)?,
            }));
        }

        let operator = Operation::try_from(type_id).ok()?;
        let length_type_id = self.next()?;

        if length_type_id {
            let num_sub_packets = self.next_n(11)?;
            let mut sub_packets = Vec::with_capacity(num_sub_packets);
            for _ in 0..num_sub_packets {
                sub_packets.push(self.parse_packet()?);
            }
            return Some(Packet::Operator(OperatorPacket {
                version,
                operator,
                sub_packets,
            }));
        }

        let target_bit_idx = self.next_n(15)? + self.bits_read();

        let mut sub_packets = Vec::new();
        while self.bits_read() < target_bit_idx {
            sub_packets.push(self.parse_packet()?);
        }
        assert_eq!(self.bits_read(), target_bit_idx);
        Some(Packet::Operator(OperatorPacket {
            version,
            operator,
            sub_packets,
        }))
    }

    fn next_n(&mut self, n: u8) -> Option<usize> {
        let mut out = self.next()? as usize;

        for _ in 0..(n - 1) {
            out <<= 1;
            match self.next() {
                Some(true) => out |= 1,
                Some(false) => {}
                None => return Some(out),
            };
        }

        Some(out)
    }

    fn bits_read(&self) -> usize {
        self.byte_idx * 8 + (7 - self.bit_idx as usize)
    }
}

impl Iterator for Biterator {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.byte_idx >= self.contents.len() {
            return None;
        }

        let out = self.contents[self.byte_idx] & (1 << self.bit_idx) > 0;

        if self.bit_idx == 0 {
            self.bit_idx = 7;
            self.byte_idx += 1;
        } else {
            self.bit_idx -= 1;
        }

        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    mod total {
        use super::*;

        #[bench]
        fn actual(b: &mut Bencher) {
            b.iter(|| {
                let data = read_data("../../data");
                assert_ne!(data, Input::default());
                assert_eq!(PART_1, part_1(&data));
                assert_eq!(PART_2, part_2(&data));
            })
        }
    }

    mod read_data {
        use super::*;

        #[bench]
        fn actual(b: &mut Bencher) {
            b.iter(|| {
                let data = read_data("../../data");

                assert_ne!(data, Packet::Empty);
            })
        }
    }

    mod parse_contents {
        use super::*;

        struct Case<'c> {
            input: &'c str,
            expected: Packet,
        }

        #[test]
        fn literal() {
            run(&Case {
                input: "D2FE28",
                expected: Packet::Literal(LiteralPacket {
                    version: 6,
                    value: 2021,
                }),
            });
        }

        #[test]
        fn operator_type_0() {
            run(&Case {
                input: "38006F45291200",
                expected: Packet::Operator(OperatorPacket {
                    version: 1,
                    operator: Operation::LessThan,
                    sub_packets: vec![
                        Packet::Literal(LiteralPacket {
                            version: 6,
                            value: 10,
                        }),
                        Packet::Literal(LiteralPacket {
                            version: 2,
                            value: 20,
                        }),
                    ],
                }),
            });
        }

        #[test]
        fn operator_type_1() {
            run(&Case {
                input: "EE00D40C823060",
                expected: Packet::Operator(OperatorPacket {
                    version: 7,
                    operator: Operation::Maximum,
                    sub_packets: vec![
                        Packet::Literal(LiteralPacket {
                            version: 2,
                            value: 1,
                        }),
                        Packet::Literal(LiteralPacket {
                            version: 4,
                            value: 2,
                        }),
                        Packet::Literal(LiteralPacket {
                            version: 1,
                            value: 3,
                        }),
                    ],
                }),
            });
        }

        #[test]
        fn example() {
            run(&Case {
                input: "8A004A801A8002F478",
                expected: Packet::Operator(OperatorPacket {
                    version: 4,
                    operator: Operation::Minimum,
                    sub_packets: vec![Packet::Operator(OperatorPacket {
                        version: 1,
                        operator: Operation::Minimum,
                        sub_packets: vec![Packet::Operator(OperatorPacket {
                            version: 5,
                            operator: Operation::Minimum,
                            sub_packets: vec![Packet::Literal(LiteralPacket {
                                version: 6,
                                value: 15,
                            })],
                        })],
                    })],
                }),
            })
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, Input::from(test.input))
        }
    }

    mod part_1 {
        use super::*;

        struct Case {
            data: Packet,
            expected: usize,
        }

        #[test]
        fn example_1() {
            run(&Case {
                data: Input::from("8A004A801A8002F478"),
                expected: 16,
            })
        }

        #[test]
        fn example_2() {
            run(&Case {
                data: Input::from("620080001611562C8802118E34"),
                expected: 12,
            })
        }

        #[test]
        fn example_3() {
            run(&Case {
                data: Input::from("C0015000016115A2E0802F182340"),
                expected: 23,
            })
        }

        #[test]
        fn example_4() {
            run(&Case {
                data: Input::from("A0016C880162017C3686B18A3D4780"),
                expected: 31,
            })
        }

        #[bench]
        fn actual(b: &mut Bencher) {
            let case = Case {
                data: read_data("../../data"),
                expected: PART_1,
            };

            b.iter(|| run(&case))
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, part_1(&test.data))
        }
    }

    mod part_2 {
        use super::*;

        struct Case {
            data: Packet,
            expected: usize,
        }

        #[test]
        fn sum() {
            run(&Case {
                data: Input::from("C200B40A82"),
                expected: 3,
            });
        }

        #[test]
        fn product() {
            run(&Case {
                data: Input::from("04005AC33890"),
                expected: 54,
            });
        }

        #[test]
        fn minimum() {
            run(&Case {
                data: Input::from("880086C3E88112"),
                expected: 7,
            });
        }

        #[test]
        fn maximum() {
            run(&Case {
                data: Input::from("CE00C43D881120"),
                expected: 9,
            });
        }

        #[test]
        fn less_than() {
            run(&Case {
                data: Input::from("D8005AC2A8F0"),
                expected: 1,
            });
        }

        #[test]
        fn greater_than() {
            run(&Case {
                data: Input::from("F600BC2D8F"),
                expected: 0,
            });
        }

        #[test]
        fn equal_to() {
            run(&Case {
                data: Input::from("9C005AC2F8F0"),
                expected: 0,
            });
        }

        #[test]
        fn nested_equal_to() {
            run(&Case {
                data: Input::from("9C0141080250320F1802104A08"),
                expected: 1,
            });
        }

        #[bench]
        fn actual(b: &mut Bencher) {
            let case = Case {
                data: read_data("../../data"),
                expected: PART_2,
            };

            b.iter(|| run(&case))
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, part_2(&test.data))
        }
    }
}
