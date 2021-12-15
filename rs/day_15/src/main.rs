#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]
#![feature(test)]
extern crate test;

use std::{fs::read_to_string, path::Path};

fn main() {
    let data = read_data("./data");

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

fn read_data(data_dir: &str) -> Input {
    read_to_string(Path::new(data_dir).join("day_15.txt"))
        .unwrap()
        .trim()
        .lines()
        .map(|l| {
            l.bytes().fold(Vec::new(), |mut acc, b| {
                acc.push((b - b'0') as usize);
                acc
            })
        })
        .collect()
}

fn part_1(input: &Input) -> usize {
    let valid_x = 0..input[0].len();
    let valid_y = 0..input.len();
    let end = ((input.len() - 1) << 9) | (input[0].len() - 1);

    let mut risks = [usize::MAX; 1 << 18];
    risks[0] = 0;

    let mut to_process = Vec::with_capacity(risks.len());
    to_process.push(0);
    while let Some(node) = to_process.pop() {
        let risk = risks[node];
        if node == end {
            return risk;
        }

        let (n_y, n_x) = (node >> 9, node & 0x1FF);

        for (x, y) in [
            (n_x, n_y - 1),
            (n_x - 1, n_y),
            (n_x + 1, n_y),
            (n_x, n_y + 1),
        ] {
            if !valid_x.contains(&x) || !valid_y.contains(&y) {
                continue;
            }
            let n = y << 9 | x;
            let new_risk = risk + input[y][x];
            if new_risk < risks[n] {
                risks[n] = new_risk;
                to_process.push(n);
            }
        }

        to_process.sort_unstable_by(|&a, &b| risks[b].cmp(&risks[a]));
    }

    panic!("no path found")
}

fn part_2(input: &Input) -> usize {
    let (in_x, in_y) = (input.len(), input[0].len());
    let mut full_map = Vec::with_capacity(5 * in_y);

    for y in 0..full_map.capacity() {
        let mut row = vec![0; 5 * in_x];
        for (x, v) in row.iter_mut().enumerate() {
            let n = input[y % in_y][x % in_x] + y / in_y + x / in_x;
            *v = n % 10 + (n / 10);
        }
        full_map.push(row);
    }
    part_1(&full_map)
}

type Input = Vec<Vec<usize>>;

#[cfg(test)]
mod day_15 {
    use super::*;
    use std::fs::write;
    use test::Bencher;

    const PART_1: usize = 472;
    const PART_2: usize = 2851;

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
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

        write(Path::new("/tmp").join("day_15.txt"), input).unwrap();

        assert_eq!(example_data(), read_data("/tmp"));
    }

    #[test]
    fn test_part_1_example() {
        let data = example_data();

        assert_eq!(40, part_1(&data));
    }

    #[test]
    fn test_part_2_example() {
        let data = example_data();

        assert_eq!(315, part_2(&data));
    }

    fn example_data() -> Input {
        vec![
            vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ]
    }

    #[bench]
    fn bench_read_data(b: &mut Bencher) {
        b.iter(|| {
            let data = read_data("../../data");

            assert_ne!(data, Input::new());
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
