#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]
#![feature(test)]
extern crate test;

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
mod day_1 {
    use super::*;
    use test::Bencher;

    const PART_1: usize = 1374;
    const PART_2: usize = 1418;

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
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(7, part_1(&data));
    }

    #[test]
    fn test_part_2_example() {
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(5, part_2(&data));
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
