#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]
#![feature(test)]
extern crate test;
use std::{fs::read_to_string, path::Path, str::FromStr};

fn main() {
    let data = read_data("./data");

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Instruction {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, distance_str) = s.split_once(' ').unwrap();

        let distance = distance_str.parse::<usize>().unwrap();
        match instruction {
            "forward" => Ok(Instruction::Forward(distance)),
            "down" => Ok(Instruction::Down(distance)),
            "up" => Ok(Instruction::Up(distance)),
            _ => Err(format!("unknown instruction: {}", instruction)),
        }
    }
}

fn read_data(data_dir: &str) -> Vec<Instruction> {
    read_to_string(Path::new(data_dir).join("day_2.txt"))
        .unwrap()
        .trim()
        .lines()
        .map(|l| Instruction::from_str(l).unwrap())
        .collect()
}

fn part_1(input: &[Instruction]) -> usize {
    let (horizontal, depth) = input.iter().fold(
        (0usize, 0usize),
        |(mut horizontal, mut depth), instruction| {
            match instruction {
                Instruction::Forward(n) => horizontal += n,
                Instruction::Down(n) => depth += n,
                Instruction::Up(n) => depth -= n,
            }
            (horizontal, depth)
        },
    );
    horizontal * depth
}

fn part_2(input: &[Instruction]) -> usize {
    let (horizontal, depth, _) = input.iter().fold(
        (0usize, 0usize, 0usize),
        |(mut horizontal, mut depth, mut aim), instruction| {
            match instruction {
                Instruction::Forward(n) => {
                    horizontal += n;
                    depth += aim * n;
                }
                Instruction::Down(n) => aim += n,
                Instruction::Up(n) => aim -= n,
            }
            (horizontal, depth, aim)
        },
    );
    horizontal * depth
}

#[cfg(test)]
mod day_2 {
    use super::*;
    use test::Bencher;

    const PART_1: usize = 1840243;
    const PART_2: usize = 1727785422;

    #[test]
    fn test_part_1_real() {
        let data = read_data("../../data");

        assert_eq!(PART_1, part_1(&data));
    }

    #[test]
    fn test_part_2_real() {
        let data = read_data("../../data");

        assert_eq!(PART_2, part_2(&data));
    }

    #[test]
    fn test_part_1_example() {
        let data = vec![
            Instruction::Forward(5),
            Instruction::Down(5),
            Instruction::Forward(8),
            Instruction::Up(3),
            Instruction::Down(8),
            Instruction::Forward(2),
        ];

        assert_eq!(150, part_1(&data));
    }

    #[test]
    fn test_part_2_example() {
        let data = vec![
            Instruction::Forward(5),
            Instruction::Down(5),
            Instruction::Forward(8),
            Instruction::Up(3),
            Instruction::Down(8),
            Instruction::Forward(2),
        ];

        assert_eq!(900, part_2(&data));
    }

    #[bench]
    fn bench_read_data(b: &mut Bencher) {
        b.iter(|| {
            let data = read_data("../../data");

            assert_ne!(data, Vec::new());
        })
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let data = read_data("../../data");

        b.iter(|| {
            assert_eq!(PART_1, part_1(&data));
        })
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let data = read_data("../../data");

        b.iter(|| {
            assert_eq!(PART_2, part_2(&data));
        })
    }
}
