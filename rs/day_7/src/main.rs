#![deny(clippy::all)]
#![feature(test)]
extern crate test;

use std::{fs::read_to_string, path::Path};

type Input = Vec<usize>;

fn main() {
    let data = read_data("./data");

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

fn read_data(data_dir: &str) -> Input {
    let contents = read_to_string(Path::new(data_dir).join("day_7.txt")).unwrap();

    parse_contents(contents.trim())
}

fn parse_contents(contents: &str) -> Input {
    let mut crabs: Input = contents
        .split(',')
        .map(|num| num.parse::<usize>().unwrap())
        .collect();
    crabs.sort_unstable();
    crabs
}

fn part_1(input: &Input) -> usize {
    let target = input[input.len() / 2];
    input.iter().fold(0usize, |acc, &ship| {
        acc + (ship as isize - target as isize).abs() as usize
    })
}

fn part_2(input: &Input) -> usize {
    (0..*input.last().unwrap()).fold(usize::MAX, |total, target| {
        total.min(input.iter().fold(0usize, |subtotal, &crab| {
            let diff = (crab as isize - target as isize).abs() as usize;
            subtotal + ((diff * (diff + 1)) / 2)
        }))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const PART_1: usize = 340056;
    const PART_2: usize = 96592275;

    mod read_data {
        use super::*;

        #[bench]
        fn actual(b: &mut Bencher) {
            b.iter(|| {
                let data = read_data("../../data");

                assert_ne!(data, Input::default())
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
                expected: 37,
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
                expected: 168,
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
        vec![0, 1, 1, 2, 2, 2, 4, 7, 14, 16]
    }
}
