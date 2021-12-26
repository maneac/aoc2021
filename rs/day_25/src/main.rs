#![deny(clippy::all)]
#![feature(test)]
extern crate test;

use std::{fs::read_to_string, path::Path};

type Input = Vec<Vec<Option<bool>>>;

fn main() {
    let data = read_data("./data");

    println!("Part 1: {}", part_1(&data));
}

fn read_data(data_dir: &str) -> Input {
    let contents = read_to_string(Path::new(data_dir).join("day_25.txt")).unwrap();

    parse_contents(contents.trim())
}

fn parse_contents(contents: &str) -> Input {
    contents.lines().fold(Vec::new(), |mut rows, line| {
        let row = line.chars().fold(Vec::new(), |mut row, chr| {
            match chr {
                '>' => row.push(Some(false)),
                'v' => row.push(Some(true)),
                _ => row.push(None),
            }
            row
        });
        rows.push(row);
        rows
    })
}

fn part_1(input: &Input) -> usize {
    let mut cucumbers = input.clone();

    let max_x = cucumbers[0].len() - 1;
    let max_y = cucumbers.len() - 1;

    for step in 1.. {
        let mut moved = false;
        let mut new_cucumbers = cucumbers.clone();

        for y in 0..=max_y {
            for x in 0..max_x {
                if let Some(false) = cucumbers[y][x] {
                    if cucumbers[y][x + 1].is_none() {
                        moved = true;
                        new_cucumbers[y][x] = None;
                        new_cucumbers[y][x + 1] = Some(false);
                    }
                }
            }
            if let Some(false) = cucumbers[y][max_x] {
                if cucumbers[y][0].is_none() {
                    moved = true;
                    new_cucumbers[y][max_x] = None;
                    new_cucumbers[y][0] = Some(false);
                }
            }
        }

        cucumbers = new_cucumbers.clone();

        for x in 0..=max_x {
            for y in 0..max_y {
                if let Some(true) = cucumbers[y][x] {
                    if cucumbers[y + 1][x].is_none() {
                        moved = true;
                        new_cucumbers[y][x] = None;
                        new_cucumbers[y + 1][x] = Some(true);
                    }
                }
            }
            if let Some(true) = cucumbers[max_y][x] {
                if cucumbers[0][x].is_none() {
                    moved = true;
                    new_cucumbers[max_y][x] = None;
                    new_cucumbers[0][x] = Some(true);
                }
            }
        }

        if !moved {
            return step;
        }
        cucumbers = new_cucumbers;
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const PART_1: usize = 453;

    mod read_data {
        use super::*;

        #[bench]
        fn actual(b: &mut Bencher) {
            b.iter(|| {
                let data = read_data("../../data");

                assert_ne!(data, Input::default())
            })
        }
    }

    mod parse_contents {
        use super::*;

        struct Case<'c> {
            input: &'c str,
            expected: Input,
        }

        #[test]
        fn example() {
            run(&Case {
                input: "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>",
                expected: example_data(),
            })
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, parse_contents(test.input))
        }
    }

    mod part_1 {
        use super::*;

        struct Case {
            data: Input,
            expected: usize,
        }

        #[test]
        fn example() {
            run(&Case {
                data: example_data(),
                expected: 58,
            })
        }

        #[bench]
        fn actual(b: &mut Bencher) {
            let case = Case {
                data: read_data("../../data"),
                expected: PART_1,
            };

            b.iter(|| run(&case))
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, part_1(&test.data))
        }
    }

    fn example_data() -> Input {
        vec![
            vec![
                Some(true),
                None,
                None,
                None,
                Some(false),
                Some(false),
                None,
                Some(true),
                Some(true),
                Some(false),
            ],
            vec![
                None,
                Some(true),
                Some(true),
                Some(false),
                Some(false),
                None,
                Some(true),
                Some(true),
                None,
                None,
            ],
            vec![
                Some(false),
                Some(false),
                None,
                Some(false),
                Some(true),
                Some(false),
                None,
                None,
                None,
                Some(true),
            ],
            vec![
                Some(false),
                Some(false),
                Some(true),
                Some(false),
                Some(false),
                None,
                Some(false),
                None,
                Some(true),
                None,
            ],
            vec![
                Some(true),
                Some(false),
                Some(true),
                None,
                Some(true),
                Some(true),
                None,
                Some(true),
                None,
                None,
            ],
            vec![
                Some(false),
                None,
                Some(false),
                Some(false),
                None,
                None,
                Some(true),
                None,
                None,
                None,
            ],
            vec![
                None,
                Some(true),
                Some(true),
                None,
                None,
                Some(false),
                None,
                Some(false),
                Some(true),
                None,
            ],
            vec![
                Some(true),
                None,
                Some(true),
                None,
                None,
                Some(false),
                Some(false),
                Some(true),
                None,
                Some(true),
            ],
            vec![
                None,
                None,
                None,
                None,
                Some(true),
                None,
                None,
                Some(true),
                None,
                Some(false),
            ],
        ]
    }
}
