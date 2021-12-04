#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]

use std::{fs::read_to_string, path::Path};

fn main() {
    let data = read_data("./data");

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

fn read_data(data_dir: &str) -> Vec<Vec<bool>> {
    read_to_string(Path::new(data_dir).join("day_3.txt"))
        .unwrap()
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c.eq(&'1')).collect())
        .collect()
}

fn part_1(input: &[Vec<bool>]) -> String {
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

    (epsilon * gamma).to_string()
}

fn part_2(input: &[Vec<bool>]) -> String {
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

    (oxygen_generator_rating * co2_scrubber_rating).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;

    #[test]
    fn test_part_1_real() {
        let data = read_data("../../data");

        assert_eq!("3148794", part_1(&data));
    }

    #[test]
    fn test_part_2_real() {
        let data = read_data("../../data");

        assert_eq!("2795310", part_2(&data));
    }

    #[test]
    fn test_read_data() {
        let input = "00100
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
01010";

        let expected = vec![
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
        ];

        write("/tmp/day_3.txt", &input).unwrap();

        let data = read_data("/tmp");

        assert_eq!(expected, data);
    }

    #[test]
    fn test_part_1_example() {
        let input = vec![
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
        ];

        assert_eq!("198", part_1(&input));
    }

    #[test]
    fn test_part_2_example() {
        let input = vec![
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
        ];

        assert_eq!("230", part_2(&input));
    }
}
