#![deny(clippy::all)]
#![feature(test)]
extern crate test;

use std::{fs::read_to_string, path::Path, str::FromStr};

type Input = Vec<Instruction>;

fn main() {
    let data = read_data("./data");

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

fn read_data(data_dir: &str) -> Input {
    let contents = read_to_string(Path::new(data_dir).join("day_2.txt")).unwrap();

    parse_contents(contents.trim())
}

fn parse_contents(contents: &str) -> Input {
    contents
        .lines()
        .map(|l| Instruction::from_str(l).unwrap())
        .collect()
}

fn part_1(input: &Input) -> usize {
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

fn part_2(input: &Input) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const PART_1: usize = 1840243;
    const PART_2: usize = 1727785422;

    mod read_data {
        use super::*;

        #[bench]
        fn actual(b: &mut Bencher) {
            b.iter(|| {
                let data = read_data("../../data");

                assert_ne!(data, Input::new())
            })
        }
    }

    mod part_1 {
        use super::*;

        struct Case {
            data: Input,
            expected: usize,
        }

        #[test]
        fn example() {
            run(&Case {
                data: example_data(),
                expected: 150,
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
            data: Input,
            expected: usize,
        }

        #[test]
        fn example() {
            run(&Case {
                data: example_data(),
                expected: 900,
            })
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

    fn example_data() -> Input {
        vec![
            Instruction::Forward(5),
            Instruction::Down(5),
            Instruction::Forward(8),
            Instruction::Up(3),
            Instruction::Down(8),
            Instruction::Forward(2),
        ]
    }
}
