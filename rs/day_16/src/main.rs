#![deny(clippy::all)]
#![feature(test)]
extern crate test;

use std::{fs::read_to_string, path::Path};

fn main() {
    let data = read_data("./data");

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

fn read_data(data_dir: &str) -> Input {
    Input::from(
        read_to_string(Path::new(data_dir).join("day_16.txt"))
            .unwrap()
            .trim(),
    )
}

fn part_1(input: &Input) -> usize {
    input.version_sum()
}

fn part_2(input: &Input) -> usize {
    input.evaluate()
}

type Input = Packet;

#[derive(Debug, PartialEq, PartialOrd)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Packet {
    Empty,
    Operator(OperatorPacket),
    Literal(LiteralPacket),
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

#[derive(Debug, PartialEq, PartialOrd, Default)]
struct LiteralPacket {
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

#[derive(Debug, PartialEq, PartialOrd)]
struct OperatorPacket {
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

#[cfg(test)]
mod day_16;

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
