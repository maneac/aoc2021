#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]
#![feature(test)]
extern crate test;
use std::{fs::read_to_string, path::Path};

fn main() {
    let data = read_data("./data");

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Input {
    vectors: Vec<[(usize, usize); 2]>,
    max: (usize, usize),
}

fn read_data(data_dir: &str) -> Input {
    let vectors: Vec<[(usize, usize); 2]> = read_to_string(Path::new(data_dir).join("day_5.txt"))
        .unwrap()
        .lines()
        .map(|l| {
            l.split_once(" -> ")
                .map(|(from_str, to_str)| {
                    let from = from_str
                        .split_once(',')
                        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                        .unwrap();

                    let to = to_str
                        .split_once(',')
                        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                        .unwrap();

                    [from, to]
                })
                .unwrap()
        })
        .collect();

    let max = vectors.iter().fold(
        (0, 0),
        |(max_x, max_y), &[(from_x, from_y), (to_x, to_y)]| {
            (max_x.max(from_x).max(to_x), max_y.max(from_y).max(to_y))
        },
    );

    Input { vectors, max }
}

fn part_1(input: &Input) -> usize {
    let row_width = input.max.0 - 1;
    input
        .vectors
        .iter()
        .filter(|[(from_x, from_y), (to_x, to_y)]| from_x.eq(to_x) || from_y.eq(to_y))
        .fold::<Vec<Option<usize>>, _>(
            vec![None; input.max.0 * input.max.1],
            |mut counts, &[(from_x, from_y), (to_x, to_y)]| {
                for x in from_x.min(to_x)..=from_x.max(to_x) {
                    for y in from_y.min(to_y)..=from_y.max(to_y) {
                        let idx = (x * row_width) + y;
                        counts[idx] = if let Some(count) = counts[idx] {
                            Some(count + 1)
                        } else {
                            Some(1)
                        };
                    }
                }
                counts
            },
        )
        .iter()
        .filter(|&&v| matches!(v, Some(n) if n > 1))
        .count()
}

fn part_2(input: &Input) -> usize {
    let row_width = input.max.0 - 1;
    input
        .vectors
        .iter()
        .fold::<Vec<Option<usize>>, _>(
            vec![None; input.max.0 * input.max.1],
            |mut counts, &[(from_x, from_y), (to_x, to_y)]| {
                let (x_inc, y_inc) = (
                    -1 + (to_x >= from_x) as isize + (to_x > from_x) as isize,
                    -1 + (to_y >= from_y) as isize + (to_y > from_y) as isize,
                );

                if x_inc == 0 || y_inc == 0 {
                    for x in from_x.min(to_x)..=from_x.max(to_x) {
                        for y in from_y.min(to_y)..=from_y.max(to_y) {
                            let idx = (x * row_width) + y;
                            counts[idx] = if let Some(count) = counts[idx] {
                                Some(count + 1)
                            } else {
                                Some(1)
                            };
                        }
                    }
                    return counts;
                }

                for i in 0..=(from_x.max(to_x) - from_x.min(to_x)) as isize {
                    let idx = ((from_x as isize + (i * x_inc)) * row_width as isize) as usize
                        + (from_y as isize + (i * y_inc)) as usize;
                    counts[idx] = if let Some(count) = counts[idx] {
                        Some(count + 1)
                    } else {
                        Some(1)
                    };
                }

                counts
            },
        )
        .iter()
        .filter(|&&v| matches!(v, Some(n) if n > 1))
        .count()
}

#[cfg(test)]
mod day_5 {
    use super::*;
    use std::fs::write;
    use test::Bencher;

    const PART_1: usize = 5608;
    const PART_2: usize = 20299;

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
    fn test_read_data() {
        let contents = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        let expected = Input {
            vectors: vec![
                [(0, 9), (5, 9)],
                [(8, 0), (0, 8)],
                [(9, 4), (3, 4)],
                [(2, 2), (2, 1)],
                [(7, 0), (7, 4)],
                [(6, 4), (2, 0)],
                [(0, 9), (2, 9)],
                [(3, 4), (1, 4)],
                [(0, 0), (8, 8)],
                [(5, 5), (8, 2)],
            ],
            max: (9, 9),
        };

        write(Path::new("/tmp").join("day_5.txt"), contents).unwrap();

        assert_eq!(expected, read_data("/tmp"))
    }

    #[test]
    fn test_part_1_example() {
        let data = Input {
            vectors: vec![
                [(0, 9), (5, 9)],
                [(8, 0), (0, 8)],
                [(9, 4), (3, 4)],
                [(2, 2), (2, 1)],
                [(7, 0), (7, 4)],
                [(6, 4), (2, 0)],
                [(0, 9), (2, 9)],
                [(3, 4), (1, 4)],
                [(0, 0), (8, 8)],
                [(5, 5), (8, 2)],
            ],
            max: (9, 9),
        };

        assert_eq!(5, part_1(&data));
    }

    #[test]
    fn test_part_2_example() {
        let data = Input {
            vectors: vec![
                [(0, 9), (5, 9)],
                [(8, 0), (0, 8)],
                [(9, 4), (3, 4)],
                [(2, 2), (2, 1)],
                [(7, 0), (7, 4)],
                [(6, 4), (2, 0)],
                [(0, 9), (2, 9)],
                [(3, 4), (1, 4)],
                [(0, 0), (8, 8)],
                [(5, 5), (8, 2)],
            ],
            max: (9, 9),
        };

        assert_eq!(12, part_2(&data));
    }

    #[bench]
    fn bench_read_data(b: &mut Bencher) {
        b.iter(|| {
            let data = read_data("../../data");

            assert_ne!(
                data,
                Input {
                    vectors: Vec::new(),
                    max: (0, 0),
                }
            );
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
