#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]
#![feature(test)]
extern crate test;

use std::{cmp::Ordering, fs::read_to_string, path::Path};

fn main() {
    let data = read_data("./data");

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

fn read_data(data_dir: &str) -> Input {
    let contents = read_to_string(Path::new(data_dir).join("day_13.txt")).unwrap();

    let (point_strs, fold_strs) = contents.trim().split_once("\n\n").unwrap();

    let mut points: Vec<(usize, usize)> = point_strs
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect();
    points.sort_unstable_by(|a, b| {
        let y_diff = a.1.cmp(&b.1);
        if y_diff != Ordering::Equal {
            y_diff
        } else {
            a.0.cmp(&b.0)
        }
    });

    let folds = fold_strs
        .lines()
        .map(|line| {
            let (axis, coordinate) = line
                .trim_start_matches("fold along ")
                .split_once('=')
                .unwrap();
            match axis {
                "x" => (coordinate.parse::<usize>().unwrap(), 0),
                "y" => (0, coordinate.parse::<usize>().unwrap()),
                _ => panic!("unknown coordinate: {}", axis),
            }
        })
        .collect();

    Input { points, folds }
}

fn part_1(input: &Input) -> usize {
    let fold = input.folds.first().unwrap();

    let mut points = input.points.clone();

    points
        .iter_mut()
        .filter(|(x, y)| x.ge(&fold.0) && y.ge(&fold.1))
        .for_each(|(x, y)| {
            if fold.0 > 0 {
                *x = (2 * fold.0) - *x
            } else {
                *y = (2 * fold.1) - *y
            }
        });

    points.sort_unstable();
    points.dedup();
    points.len()
}

fn part_2(input: &Input) -> String {
    let mut points = input.points.clone();
    for fold in &input.folds {
        points
            .iter_mut()
            .filter(|(x, y)| x.ge(&fold.0) && y.ge(&fold.1))
            .for_each(|(x, y)| {
                if fold.0 > 0 {
                    *x = (2 * fold.0) - *x
                } else {
                    *y = (2 * fold.1) - *y
                }
            });

        points.sort_unstable();
        points.dedup();
    }

    let (max_x, max_y) = points.iter().fold((0usize, 0usize), |acc, &(x, y)| {
        (acc.0.max(x), acc.1.max(y))
    });

    let num_chars = (max_x + 4) / 5;

    let display: Vec<Vec<char>> = points.iter().fold(
        vec![vec!['.'; 5 * num_chars]; max_y + 1],
        |mut acc, &(x, y)| {
            acc[y][x] = '#';
            acc
        },
    );

    let chars = display.iter().fold(
        vec![String::with_capacity((max_y + 1) * 5); num_chars],
        |mut acc, line| {
            line.chunks(5).enumerate().for_each(|(char_idx, chunk)| {
                dbg!(chunk.len());
                for &char in chunk {
                    acc[char_idx].push(char);
                }
                acc[char_idx].push('\n');
            });
            acc
        },
    );

    chars
        .iter()
        .map(|chr| {
            b'a' + LETTERS
                .iter()
                .position(|l| !l.is_empty() && &l[1..] == chr)
                .unwrap() as u8
        })
        .fold(String::new(), |mut acc, chr| {
            acc.push(chr as char);
            acc
        })
}

#[derive(Debug, PartialEq, PartialOrd, Default)]
struct Input {
    points: Vec<(usize, usize)>,
    folds: Vec<(usize, usize)>,
}

// missing letters obtained from https://gist.github.com/Aidiakapi/3d8e32c3e552ca1258494cc114d8acf5
// and https://www.reddit.com/r/adventofcode/comments/5h52ro/comment/daxv8cr/?utm_source=share&utm_medium=web2x&context=3
const LETTERS: [&str; 27] = [
    "
.##..
#..#.
#..#.
####.
#..#.
#..#.
",
    "
###..
#..#.
###..
#..#.
#..#.
###..
",
    "
.##..
#..#.
#....
#....
#..#.
.##..
",
    "
###..
#..#.
#..#.
#..#.
#..#.
###..
",
    "
####.
#....
###..
#....
#....
####.
",
    "
####.
#....
###..
#....
#....
#....
",
    "
.##..
#..#.
#....
#.##.
#..#.
.###.
",
    "
#..#.
#..#.
####.
#..#.
#..#.
#..#.
",
    "
.###.
..#..
..#..
..#..
..#..
.###.
",
    "
..##.
...#.
...#.
...#.
#..#.
.##..
",
    "
#..#.
#.#..
##...
#.#..
#.#..
#..#.
",
    "
#....
#....
#....
#....
#....
####.
",
    // M
    "",
    // N
    "",
    "
.##..
#..#.
#..#.
#..#.
#..#.
.##..
",
    "
###..
#..#.
#..#.
###..
#....
#....
",
    // Q
    "",
    "
###..
#..#.
#..#.
###..
#.#..
#..#.
",
    "
.###.
#....
#....
.##..
...#.
###..
",
    "
.###.
..#..
..#..
..#..
..#..
..#..
",
    "
#..#.
#..#.
#..#.
#..#.
#..#.
.##..
",
    // V
    "",
    // W
    "",
    // X
    "",
    "
#...#
#...#
.#.#.
..#..
..#..
..#..
",
    "
####.
...#.
..#..
.#...
#....
####.
",
    // special case for example
    "
#####
#...#
#...#
#...#
#####
",
];

#[cfg(test)]
mod day_13 {
    use super::*;
    use std::fs::write;
    use test::Bencher;

    const PART_1: usize = 669;
    const PART_2: &str = "uefzcucj";

    #[test]
    fn test_part_1_real() {
        let data = read_data("../../data");

        assert_eq!(PART_1, part_1(&data));
    }

    #[test]
    fn test_part_2_real() {
        let data = read_data("../../data");

        assert_eq!(PART_2, part_2(&data).as_str());
    }

    #[test]
    fn test_read_data() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

        write(Path::new("/tmp").join("day_13.txt"), input).unwrap();

        assert_eq!(example_data(), read_data("/tmp"));
    }

    #[test]
    fn test_part_1_example() {
        let data = example_data();

        assert_eq!(17, part_1(&data));
    }

    #[test]
    fn test_part_2_example() {
        let data = example_data();

        // '{' = 'z'+1
        assert_eq!("{", part_2(&data).as_str());
    }

    fn example_data() -> Input {
        Input {
            points: vec![
                (3, 0),
                (6, 0),
                (9, 0),
                (4, 1),
                (0, 3),
                (3, 4),
                (8, 4),
                (10, 4),
                (1, 10),
                (6, 10),
                (8, 10),
                (9, 10),
                (4, 11),
                (6, 12),
                (10, 12),
                (0, 13),
                (0, 14),
                (2, 14),
            ],
            folds: vec![(0, 7), (5, 0)],
        }
    }

    #[bench]
    fn bench_read_data(b: &mut Bencher) {
        b.iter(|| {
            let data = read_data("../../data");

            assert_ne!(data, Input::default());
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
