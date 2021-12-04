#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]

use std::{fs::read_to_string, path::Path, str::FromStr};

fn main() {
    let data = read_data("./data");

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

enum Instruction {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, distance_str) = s.split_once(" ").unwrap();

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

fn part_1(input: &[Instruction]) -> String {
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
    (horizontal * depth).to_string()
}

fn part_2(input: &[Instruction]) -> String {
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
    (horizontal * depth).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_real() {
        let data = read_data("../../data");

        assert_eq!("1840243", part_1(&data));
    }

    #[test]
    fn test_part_2_real() {
        let data = read_data("../../data");

        assert_eq!("1727785422", part_2(&data));
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

        assert_eq!("150", part_1(&data));
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

        assert_eq!("900", part_2(&data));
    }
}
