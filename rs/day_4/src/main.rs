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
    let contents = read_to_string(Path::new(data_dir).join("day_4.txt")).unwrap();

    let mut day_parts = contents.split("\n\n");

    let numbers = day_parts
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    let boards = day_parts.map(board_from_str).collect();

    Input { numbers, boards }
}

fn part_1(input: &Input) -> usize {
    let mut boards = input.boards.clone();
    for num in &input.numbers {
        for board in boards.iter_mut() {
            board.iter_mut().for_each(|v| {
                if let Some(n) = v {
                    if n == num {
                        *v = None
                    }
                }
            });

            for i in 0..5 {
                let row_has_won = board.iter().skip(5 * i).take(5).all(|v| v.is_none());
                let col_has_won = board.iter().skip(i).step_by(5).all(|v| v.is_none());

                if row_has_won || col_has_won {
                    return board.iter().filter_map(|v| *v).sum::<usize>() * num;
                }
            }
        }
    }
    panic!("no result")
}

fn part_2(input: &Input) -> usize {
    let mut boards = input.boards.clone();
    for num in &input.numbers {
        boards.iter_mut().for_each(|b| {
            b.iter_mut().for_each(|v| match v {
                Some(n) if n == num => *v = None,
                _ => {}
            })
        });

        let new_boards = boards
            .iter()
            .filter_map(|&board| {
                for i in 0..5 {
                    let row_has_won = board.iter().skip(5 * i).take(5).all(|v| v.is_none());
                    let col_has_won = board.iter().skip(i).step_by(5).all(|v| v.is_none());

                    if row_has_won || col_has_won {
                        return None;
                    };
                }
                Some(board)
            })
            .collect::<Vec<Board>>();

        if new_boards.is_empty() {
            return boards[0].iter().filter_map(|v| *v).sum::<usize>() * num;
        }
        boards = new_boards;
    }
    panic!("no result")
}

type Board = [Option<usize>; 25];

#[derive(Debug, PartialEq, PartialOrd)]
struct Input {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

fn board_from_str(input: &str) -> Board {
    let mut b = [None; 25];
    input
        .split_ascii_whitespace()
        .enumerate()
        .for_each(|(idx, v)| b[idx] = Some(v.parse::<usize>().unwrap()));
    b
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;
    use test::Bencher;

    const PART_1: usize = 8580;
    const PART_2: usize = 9576;

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
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";

        let expected = example_input();

        write("/tmp/day_4.txt", &input).unwrap();

        assert_eq!(expected, read_data("/tmp"));
    }

    #[test]
    fn test_part_1_example() {
        let input = example_input();

        assert_eq!(4512, part_1(&input))
    }

    #[test]
    fn test_part_2_example() {
        let input = example_input();

        assert_eq!(1924, part_2(&input))
    }

    fn example_input() -> Input {
        Input {
            numbers: vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ],
            boards: vec![
                [
                    Some(22),
                    Some(13),
                    Some(17),
                    Some(11),
                    Some(0),
                    Some(8),
                    Some(2),
                    Some(23),
                    Some(4),
                    Some(24),
                    Some(21),
                    Some(9),
                    Some(14),
                    Some(16),
                    Some(7),
                    Some(6),
                    Some(10),
                    Some(3),
                    Some(18),
                    Some(5),
                    Some(1),
                    Some(12),
                    Some(20),
                    Some(15),
                    Some(19),
                ],
                [
                    Some(3),
                    Some(15),
                    Some(0),
                    Some(2),
                    Some(22),
                    Some(9),
                    Some(18),
                    Some(13),
                    Some(17),
                    Some(5),
                    Some(19),
                    Some(8),
                    Some(7),
                    Some(25),
                    Some(23),
                    Some(20),
                    Some(11),
                    Some(10),
                    Some(24),
                    Some(4),
                    Some(14),
                    Some(21),
                    Some(16),
                    Some(12),
                    Some(6),
                ],
                [
                    Some(14),
                    Some(21),
                    Some(17),
                    Some(24),
                    Some(4),
                    Some(10),
                    Some(16),
                    Some(15),
                    Some(9),
                    Some(19),
                    Some(18),
                    Some(8),
                    Some(23),
                    Some(26),
                    Some(20),
                    Some(22),
                    Some(11),
                    Some(13),
                    Some(6),
                    Some(5),
                    Some(2),
                    Some(0),
                    Some(12),
                    Some(3),
                    Some(7),
                ],
            ],
        }
    }

    #[bench]
    fn bench_read_data(b: &mut Bencher) {
        b.iter(|| {
            let data = read_data("../../data");

            assert_ne!(
                data,
                Input {
                    numbers: Vec::new(),
                    boards: Vec::new(),
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
