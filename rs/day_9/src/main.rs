#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]
#![feature(test)]
extern crate test;

use std::{fs::read_to_string, path::Path};

fn main() {
    let data = read_data("./data");

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

fn read_data(data_dir: &str) -> Vec<Vec<u8>> {
    read_to_string(Path::new(data_dir).join("day_9.txt"))
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
        .collect()
}

fn part_1(input: &[Vec<u8>]) -> usize {
    input.iter().enumerate().fold(0usize, |acc, (y, row)| {
        acc + row
            .iter()
            .enumerate()
            .fold(0usize, |row_acc, (x, &height)| {
                if y > 0 {
                    if let Some(up) = input.get(y - 1).and_then(|prev_row| prev_row.get(x)) {
                        if height.ge(up) {
                            return row_acc;
                        }
                    }
                }
                if let Some(down) = input.get(y + 1).and_then(|prev_row| prev_row.get(x)) {
                    if height.ge(down) {
                        return row_acc;
                    }
                }
                if x > 0 {
                    if let Some(left) = row.get(x - 1) {
                        if height.ge(left) {
                            return row_acc;
                        }
                    }
                }
                if let Some(right) = row.get(x + 1) {
                    if height.ge(right) {
                        return row_acc;
                    }
                }
                row_acc + height as usize + 1
            })
    })
}

fn part_2(input: &[Vec<u8>]) -> usize {
    let minima = find_minima(input);

    let mut heights: Vec<Vec<u8>> = input.to_vec();

    let avg_basin = (input.len() * input[0].len()) / minima.len();
    let mut basins = vec![Vec::<(usize, usize)>::with_capacity(avg_basin); minima.len()];

    let mut to_visit: Vec<(usize, usize)> = Vec::with_capacity(avg_basin);

    for (idx, &(minima_x, minima_y)) in minima.iter().enumerate() {
        to_visit.push((minima_x, minima_y));
        while !to_visit.is_empty() {
            let visiting = to_visit.clone();
            to_visit.clear();

            for (x, y) in visiting {
                basins[idx].push((x, y));
                heights[y][x] = 9;

                if y > 0 {
                    if let Some(&up) = heights.get(y - 1).and_then(|prev_row| prev_row.get(x)) {
                        if up != 9 {
                            to_visit.push((x, y - 1));
                        }
                    }
                }
                if let Some(&down) = heights.get(y + 1).and_then(|prev_row| prev_row.get(x)) {
                    if down != 9 {
                        to_visit.push((x, y + 1));
                    }
                }
                if x > 0 {
                    if let Some(&left) = heights[y].get(x - 1) {
                        if left != 9 {
                            to_visit.push((x - 1, y));
                        }
                    }
                }
                if let Some(&right) = heights[y].get(x + 1) {
                    if right != 9 {
                        to_visit.push((x + 1, y));
                    }
                }
            }
        }
    }

    let mut basin_sizes: Vec<usize> = basins
        .iter_mut()
        .map(|basin| {
            basin.sort_unstable();
            basin.dedup();
            basin.len()
        })
        .collect();

    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

fn find_minima(input: &[Vec<u8>]) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (y, row)| {
            let mut row_minima =
                row.iter()
                    .enumerate()
                    .fold(Vec::new(), |mut row_acc, (x, &height)| {
                        if y > 0 {
                            if let Some(up) = input.get(y - 1).and_then(|prev_row| prev_row.get(x))
                            {
                                if height.ge(up) {
                                    return row_acc;
                                }
                            }
                        }
                        if let Some(down) = input.get(y + 1).and_then(|prev_row| prev_row.get(x)) {
                            if height.ge(down) {
                                return row_acc;
                            }
                        }
                        if x > 0 {
                            if let Some(left) = row.get(x - 1) {
                                if height.ge(left) {
                                    return row_acc;
                                }
                            }
                        }
                        if let Some(right) = row.get(x + 1) {
                            if height.ge(right) {
                                return row_acc;
                            }
                        }
                        row_acc.push((x, y));
                        row_acc
                    });
            if !row_minima.is_empty() {
                acc.append(&mut row_minima);
            }
            acc
        })
}

#[cfg(test)]
mod day_9 {
    use std::fs::write;

    use super::*;
    use test::Bencher;

    const PART_1: usize = 475;
    const PART_2: usize = 1092012;

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
    fn test_parse_contents() {
        let contents = "2199943210
3987894921
9856789892
8767896789
9899965678";

        let expected = example_data();

        write(Path::new("/tmp").join("day_9.txt"), contents).unwrap();

        assert_eq!(expected, read_data("/tmp"));
    }

    #[test]
    fn test_part_1_example() {
        let data = example_data();

        assert_eq!(15, part_1(&data));
    }

    #[test]
    fn test_part_2_example() {
        let data = example_data();

        assert_eq!(1134, part_2(&data));
    }

    fn example_data() -> Vec<Vec<u8>> {
        vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ]
    }

    #[bench]
    fn bench_read_data(b: &mut Bencher) {
        b.iter(|| {
            let data = read_data("../../data");

            assert_ne!(data, Vec::<Vec<u8>>::new());
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
