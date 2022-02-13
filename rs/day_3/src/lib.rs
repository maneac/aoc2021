#![deny(clippy::all)]
#![feature(test)]
extern crate test;

use std::{fs::read_to_string, path::Path};

pub type Input = Vec<Vec<bool>>;

pub const PART_1: usize = 3148794;
pub const PART_2: usize = 2795310;

pub fn read_data(data_dir: &str) -> Input {
    let contents = read_to_string(Path::new(data_dir).join("day_3.txt")).unwrap();

    parse_contents(contents.trim())
}

fn parse_contents(contents: &str) -> Input {
    contents
        .lines()
        .map(|line| line.chars().map(|c| c.eq(&'1')).collect())
        .collect()
}

pub fn part_1(input: &Input) -> usize {
    let entry_len = input[0].len();
    let (gamma, epsilon) = input
        .iter()
        .fold(vec![0usize; entry_len], |mut acc, line| {
            acc.iter_mut()
                .zip(line.iter())
                .for_each(|(acc_entry, &line_entry)| {
                    if line_entry {
                        *acc_entry += 1;
                    }
                });
            acc
        })
        .iter()
        .rev()
        .enumerate()
        .fold(
            (0usize, 0usize),
            |(mut gamma, mut epsilon), (idx, &count)| {
                if (count << 1) >= input.len() {
                    gamma |= 1 << idx;
                } else {
                    epsilon |= 1 << idx;
                }
                (gamma, epsilon)
            },
        );

    epsilon * gamma
}

pub fn part_2(input: &Input) -> usize {
    let entry_len = input[0].len();

    let oxygen_generator_rating = {
        let mut filtered_input = input.to_owned();
        for idx in 0..entry_len {
            if filtered_input.len() == 1 {
                break;
            }
            let target = filtered_input.iter().fold(0usize, |mut acc, num| {
                if num[idx] {
                    acc += 1
                }
                acc
            }) << 1
                >= filtered_input.len();

            filtered_input = filtered_input
                .iter()
                .filter_map(|num| {
                    if num[idx] == target {
                        Some(num.to_owned())
                    } else {
                        None
                    }
                })
                .collect();
        }
        filtered_input[0]
            .iter()
            .rev()
            .enumerate()
            .fold(0usize, |mut acc, (idx, &set_bit)| {
                if set_bit {
                    acc |= 1 << idx
                }
                acc
            })
    };

    let co2_scrubber_rating = {
        let mut filtered_input = input.to_owned();
        for idx in 0..entry_len {
            if filtered_input.len() == 1 {
                break;
            }
            let target = filtered_input.iter().fold(0usize, |mut acc, num| {
                if num[idx] {
                    acc += 1
                }
                acc
            }) << 1
                < filtered_input.len();

            filtered_input = filtered_input
                .iter()
                .filter_map(|num| {
                    if num[idx] == target {
                        Some(num.to_owned())
                    } else {
                        None
                    }
                })
                .collect();
        }
        filtered_input[0]
            .iter()
            .rev()
            .enumerate()
            .fold(0usize, |mut acc, (idx, &set_bit)| {
                if set_bit {
                    acc |= 1 << idx
                }
                acc
            })
    };

    oxygen_generator_rating * co2_scrubber_rating
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

                assert_ne!(data, Input::new())
            })
        }
    }

    mod parse_contents {
        use super::*;

        struct Case<'c> {
            input: &'c str,
            expected: Input,
        }

        #[test]
        fn example() {
            run(&Case {
                input: "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
                expected: example_data(),
            })
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, parse_contents(test.input))
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
                expected: 198,
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
                expected: 230,
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
            vec![false, false, true, false, false],
            vec![true, true, true, true, false],
            vec![true, false, true, true, false],
            vec![true, false, true, true, true],
            vec![true, false, true, false, true],
            vec![false, true, true, true, true],
            vec![false, false, true, true, true],
            vec![true, true, true, false, false],
            vec![true, false, false, false, false],
            vec![true, true, false, false, true],
            vec![false, false, false, true, false],
            vec![false, true, false, true, false],
        ]
    }
}
