#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]

use std::path::Path;

fn main() {
    let data = read_data("./data");

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

fn read_data(data_dir: &str) -> Vec<usize> {
    std::fs::read_to_string(Path::new(data_dir).join("day_1.txt"))
        .unwrap()
        .trim()
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect()
}

fn part_1(input: &[usize]) -> usize {
    input
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}

fn part_2(input: &[usize]) -> usize {
    input
        .windows(3)
        .zip(input[1..].windows(3))
        .filter(|(old, new)| new.iter().sum::<usize>() > old.iter().sum())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_real() {
        let data = read_data("../../data");

        assert_eq!(1374, part_1(&data));
    }

    #[test]
    fn test_part_2_real() {
        let data = read_data("../../data");

        assert_eq!(1418, part_2(&data));
    }

    #[test]
    fn test_part_1_example() {
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(7, part_1(&data));
    }

    #[test]
    fn test_part_2_example() {
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(5, part_2(&data));
    }
}
