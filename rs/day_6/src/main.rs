#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]

use std::{fs::read_to_string, path::Path};

fn main() {
    let data = read_data("./data");

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

fn read_data(data_dir: &str) -> [usize; 9] {
    read_to_string(Path::new(data_dir).join("day_6.txt"))
        .unwrap()
        .trim()
        .split(',')
        .fold([0usize; 9], |mut acc, num| {
            acc[num.parse::<usize>().unwrap()] += 1;
            acc
        })
}

fn part_1(input: &[usize; 9]) -> usize {
    fish_after_days(input, 80)
}

fn part_2(input: &[usize; 9]) -> usize {
    fish_after_days(input, 256)
}

fn fish_after_days(input: &[usize; 9], days: usize) -> usize {
    (0..days)
        .fold(*input, |mut fish, _| {
            fish.rotate_left(1);
            fish[6] += fish[8];
            fish
        })
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;

    #[test]
    fn test_part_1_real() {
        let data = read_data("../../data");

        assert_eq!(360761, part_1(&data));
    }

    #[test]
    fn test_part_2_real() {
        let data = read_data("../../data");

        assert_eq!(1632779838045, part_2(&data));
    }

    #[test]
    fn test_read_data() {
        let input = "3,4,3,1,2";

        let expected: [usize; 9] = [0, 1, 1, 2, 1, 0, 0, 0, 0];

        write("/tmp/day_6.txt", input).unwrap();

        assert_eq!(expected, read_data("/tmp"));
    }

    #[test]
    fn test_fish_after_days() {
        let data: [usize; 9] = [0, 1, 1, 2, 1, 0, 0, 0, 0];

        assert_eq!(26, fish_after_days(&data, 18));
    }

    #[test]
    fn test_part_1_example() {
        let data: [usize; 9] = [0, 1, 1, 2, 1, 0, 0, 0, 0];

        assert_eq!(5934, part_1(&data));
    }

    #[test]
    fn test_part_2_example() {
        let data: [usize; 9] = [0, 1, 1, 2, 1, 0, 0, 0, 0];

        assert_eq!(26984457539, part_2(&data));
    }
}