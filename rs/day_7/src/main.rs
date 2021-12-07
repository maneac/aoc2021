#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]

use std::{fs::read_to_string, path::Path};

fn main() {
    let data = read_data("./data");

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

fn read_data(data_dir: &str) -> Vec<usize> {
    let mut crabs: Vec<usize> = read_to_string(Path::new(data_dir).join("day_7.txt"))
        .unwrap()
        .trim()
        .split(',')
        .map(|num| num.parse::<usize>().unwrap())
        .collect();
    crabs.sort_unstable();
    crabs
}

fn part_1(input: &[usize]) -> usize {
    let target = input[input.len() / 2];
    input.iter().fold(0usize, |acc, &ship| {
        acc + (ship as isize - target as isize).abs() as usize
    })
}

fn part_2(input: &[usize]) -> usize {
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

    #[test]
    fn test_part_1_real() {
        let data = read_data("../../data");

        assert_eq!(340056, part_1(&data));
    }

    #[test]
    fn test_part_2_real() {
        let data = read_data("../../data");

        assert_eq!(96592275, part_2(&data));
    }

    #[test]
    fn test_part_1_example() {
        let data: Vec<usize> = vec![0, 1, 1, 2, 2, 2, 4, 7, 14, 16];

        assert_eq!(37, part_1(&data))
    }

    #[test]
    fn test_part_2_example() {
        let data: Vec<usize> = vec![0, 1, 1, 2, 2, 2, 4, 7, 14, 16];

        assert_eq!(168, part_2(&data))
    }
}
