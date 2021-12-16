#![deny(clippy::all)]
#![feature(test)]
extern crate test;

use std::{
    fmt::{Display, Write},
    fs::read_to_string,
    path::Path,
};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Input {
    octopodes: [Octopus; 100],
}

fn main() {
    let data = read_data("./data");

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

fn read_data(data_dir: &str) -> Input {
    let contents = read_to_string(Path::new(data_dir).join("day_11.txt")).unwrap();

    parse_contents(contents.trim())
}

fn parse_contents(contents: &str) -> Input {
    contents.lines().enumerate().fold(
        Input {
            octopodes: [Octopus::default(); 100],
        },
        |mut acc, (y, row)| {
            row.char_indices().for_each(|(x, chr)| {
                let idx = (y * 10) + x;
                acc.octopodes[idx].value = (chr as u8 - b'0') as u8;

                let x = x as isize;
                let y = y as isize;

                acc.octopodes[idx]
                    .neighbours
                    .iter_mut()
                    .zip(
                        (((10 * (y - 1)) + x - 1)..=((10 * (y - 1)) + x + 1))
                            .chain([(10 * y) + x - 1, (10 * y) + x + 1])
                            .chain(((10 * (y + 1)) + x - 1)..=((10 * (y + 1)) + x + 1)),
                    )
                    .enumerate()
                    .for_each(|(idx, (n, target))| {
                        if !(0..=99).contains(&target)
                            || (x == 0 && (idx == 0 || idx == 3 || idx == 5))
                            || (x == 9 && (idx == 2 || idx == 4 || idx == 7))
                        {
                            return;
                        }
                        *n = target as usize;
                    });
            });
            acc
        },
    )
}

fn part_1(input: &Input) -> usize {
    let mut input = input.to_owned();
    (0..100).fold(0usize, |mut flash_count, _| {
        input
            .octopodes
            .iter_mut()
            .for_each(|octopus| octopus.value += 1);

        for i in 0..input.octopodes.len() {
            if input.octopodes[i].value > 9 {
                flash_count += input.flash(i);
            }
        }
        flash_count
    })
}

fn part_2(input: &Input) -> usize {
    let mut input = input.to_owned();
    let mut iteration = 0;
    loop {
        iteration += 1;

        input
            .octopodes
            .iter_mut()
            .for_each(|octopus| octopus.value += 1);

        for i in 0..input.octopodes.len() {
            if input.octopodes[i].value > 9 {
                input.flash(i);
            }
        }

        if input
            .octopodes
            .iter()
            .filter(|octopus| octopus.value != 0)
            .count()
            == 0
        {
            return iteration;
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct Octopus {
    value: u8,
    neighbours: [usize; 8],
}

impl Default for Octopus {
    fn default() -> Self {
        Self {
            value: 0,
            neighbours: [100; 8],
        }
    }
}

impl Input {
    fn flash(&mut self, idx: usize) -> usize {
        let mut octopus = &mut self.octopodes[idx];
        if octopus.value < 9 {
            if octopus.value > 0 {
                octopus.value += 1;
            }
            return 0;
        }
        octopus.value = 0;
        let neighbours = octopus.neighbours;
        neighbours
            .iter()
            .filter(|&n| n.le(&99))
            .fold(1usize, |flash_count, &neighbour| {
                flash_count + self.flash(neighbour)
            })
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for (idx, octopus) in self.octopodes.iter().enumerate() {
            f.write_char((octopus.value + b'0') as char)?;
            if (idx + 1) % 10 == 0 {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const PART_1: usize = 1681;
    const PART_2: usize = 276;

    mod read_data {
        use super::*;

        #[bench]
        fn actual(b: &mut Bencher) {
            b.iter(|| {
                let data = read_data("../../data");

                assert_ne!(data.octopodes[0], Octopus::default())
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
                input: "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
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
                expected: 1656,
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

    mod part_2 {
        use super::*;

        struct Case {
            data: Input,
            expected: usize,
        }

        #[test]
        fn example() {
            run(&Case {
                data: example_data(),
                expected: 195,
            })
        }

        #[bench]
        fn actual(b: &mut Bencher) {
            let case = Case {
                data: read_data("../../data"),
                expected: PART_2,
            };

            b.iter(|| run(&case))
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, part_2(&test.data))
        }
    }

    fn example_data() -> Input {
        Input {
            octopodes: [
                // 5483143223
                Octopus {
                    value: 5,
                    neighbours: [100, 100, 100, 100, 1, 100, 10, 11],
                },
                Octopus {
                    value: 4,
                    neighbours: [100, 100, 100, 0, 2, 10, 11, 12],
                },
                Octopus {
                    value: 8,
                    neighbours: [100, 100, 100, 1, 3, 11, 12, 13],
                },
                Octopus {
                    value: 3,
                    neighbours: [100, 100, 100, 2, 4, 12, 13, 14],
                },
                Octopus {
                    value: 1,
                    neighbours: [100, 100, 100, 3, 5, 13, 14, 15],
                },
                Octopus {
                    value: 4,
                    neighbours: [100, 100, 100, 4, 6, 14, 15, 16],
                },
                Octopus {
                    value: 3,
                    neighbours: [100, 100, 100, 5, 7, 15, 16, 17],
                },
                Octopus {
                    value: 2,
                    neighbours: [100, 100, 100, 6, 8, 16, 17, 18],
                },
                Octopus {
                    value: 2,
                    neighbours: [100, 100, 100, 7, 9, 17, 18, 19],
                },
                Octopus {
                    value: 3,
                    neighbours: [100, 100, 100, 8, 100, 18, 19, 100],
                },
                // 2745854711
                Octopus {
                    value: 2,
                    neighbours: [100, 0, 1, 100, 11, 100, 20, 21],
                },
                Octopus {
                    value: 7,
                    neighbours: [0, 1, 2, 10, 12, 20, 21, 22],
                },
                Octopus {
                    value: 4,
                    neighbours: [1, 2, 3, 11, 13, 21, 22, 23],
                },
                Octopus {
                    value: 5,
                    neighbours: [2, 3, 4, 12, 14, 22, 23, 24],
                },
                Octopus {
                    value: 8,
                    neighbours: [3, 4, 5, 13, 15, 23, 24, 25],
                },
                Octopus {
                    value: 5,
                    neighbours: [4, 5, 6, 14, 16, 24, 25, 26],
                },
                Octopus {
                    value: 4,
                    neighbours: [5, 6, 7, 15, 17, 25, 26, 27],
                },
                Octopus {
                    value: 7,
                    neighbours: [6, 7, 8, 16, 18, 26, 27, 28],
                },
                Octopus {
                    value: 1,
                    neighbours: [7, 8, 9, 17, 19, 27, 28, 29],
                },
                Octopus {
                    value: 1,
                    neighbours: [8, 9, 100, 18, 100, 28, 29, 100],
                },
                // 5264556173
                Octopus {
                    value: 5,
                    neighbours: [100, 10, 11, 100, 21, 100, 30, 31],
                },
                Octopus {
                    value: 2,
                    neighbours: [10, 11, 12, 20, 22, 30, 31, 32],
                },
                Octopus {
                    value: 6,
                    neighbours: [11, 12, 13, 21, 23, 31, 32, 33],
                },
                Octopus {
                    value: 4,
                    neighbours: [12, 13, 14, 22, 24, 32, 33, 34],
                },
                Octopus {
                    value: 5,
                    neighbours: [13, 14, 15, 23, 25, 33, 34, 35],
                },
                Octopus {
                    value: 5,
                    neighbours: [14, 15, 16, 24, 26, 34, 35, 36],
                },
                Octopus {
                    value: 6,
                    neighbours: [15, 16, 17, 25, 27, 35, 36, 37],
                },
                Octopus {
                    value: 1,
                    neighbours: [16, 17, 18, 26, 28, 36, 37, 38],
                },
                Octopus {
                    value: 7,
                    neighbours: [17, 18, 19, 27, 29, 37, 38, 39],
                },
                Octopus {
                    value: 3,
                    neighbours: [18, 19, 100, 28, 100, 38, 39, 100],
                },
                // 6141336146
                Octopus {
                    value: 6,
                    neighbours: [100, 20, 21, 100, 31, 100, 40, 41],
                },
                Octopus {
                    value: 1,
                    neighbours: [20, 21, 22, 30, 32, 40, 41, 42],
                },
                Octopus {
                    value: 4,
                    neighbours: [21, 22, 23, 31, 33, 41, 42, 43],
                },
                Octopus {
                    value: 1,
                    neighbours: [22, 23, 24, 32, 34, 42, 43, 44],
                },
                Octopus {
                    value: 3,
                    neighbours: [23, 24, 25, 33, 35, 43, 44, 45],
                },
                Octopus {
                    value: 3,
                    neighbours: [24, 25, 26, 34, 36, 44, 45, 46],
                },
                Octopus {
                    value: 6,
                    neighbours: [25, 26, 27, 35, 37, 45, 46, 47],
                },
                Octopus {
                    value: 1,
                    neighbours: [26, 27, 28, 36, 38, 46, 47, 48],
                },
                Octopus {
                    value: 4,
                    neighbours: [27, 28, 29, 37, 39, 47, 48, 49],
                },
                Octopus {
                    value: 6,
                    neighbours: [28, 29, 100, 38, 100, 48, 49, 100],
                },
                // 6357385478
                Octopus {
                    value: 6,
                    neighbours: [100, 30, 31, 100, 41, 100, 50, 51],
                },
                Octopus {
                    value: 3,
                    neighbours: [30, 31, 32, 40, 42, 50, 51, 52],
                },
                Octopus {
                    value: 5,
                    neighbours: [31, 32, 33, 41, 43, 51, 52, 53],
                },
                Octopus {
                    value: 7,
                    neighbours: [32, 33, 34, 42, 44, 52, 53, 54],
                },
                Octopus {
                    value: 3,
                    neighbours: [33, 34, 35, 43, 45, 53, 54, 55],
                },
                Octopus {
                    value: 8,
                    neighbours: [34, 35, 36, 44, 46, 54, 55, 56],
                },
                Octopus {
                    value: 5,
                    neighbours: [35, 36, 37, 45, 47, 55, 56, 57],
                },
                Octopus {
                    value: 4,
                    neighbours: [36, 37, 38, 46, 48, 56, 57, 58],
                },
                Octopus {
                    value: 7,
                    neighbours: [37, 38, 39, 47, 49, 57, 58, 59],
                },
                Octopus {
                    value: 8,
                    neighbours: [38, 39, 100, 48, 100, 58, 59, 100],
                },
                // 4167524645
                Octopus {
                    value: 4,
                    neighbours: [100, 40, 41, 100, 51, 100, 60, 61],
                },
                Octopus {
                    value: 1,
                    neighbours: [40, 41, 42, 50, 52, 60, 61, 62],
                },
                Octopus {
                    value: 6,
                    neighbours: [41, 42, 43, 51, 53, 61, 62, 63],
                },
                Octopus {
                    value: 7,
                    neighbours: [42, 43, 44, 52, 54, 62, 63, 64],
                },
                Octopus {
                    value: 5,
                    neighbours: [43, 44, 45, 53, 55, 63, 64, 65],
                },
                Octopus {
                    value: 2,
                    neighbours: [44, 45, 46, 54, 56, 64, 65, 66],
                },
                Octopus {
                    value: 4,
                    neighbours: [45, 46, 47, 55, 57, 65, 66, 67],
                },
                Octopus {
                    value: 6,
                    neighbours: [46, 47, 48, 56, 58, 66, 67, 68],
                },
                Octopus {
                    value: 4,
                    neighbours: [47, 48, 49, 57, 59, 67, 68, 69],
                },
                Octopus {
                    value: 5,
                    neighbours: [48, 49, 100, 58, 100, 68, 69, 100],
                },
                // 2176841721
                Octopus {
                    value: 2,
                    neighbours: [100, 50, 51, 100, 61, 100, 70, 71],
                },
                Octopus {
                    value: 1,
                    neighbours: [50, 51, 52, 60, 62, 70, 71, 72],
                },
                Octopus {
                    value: 7,
                    neighbours: [51, 52, 53, 61, 63, 71, 72, 73],
                },
                Octopus {
                    value: 6,
                    neighbours: [52, 53, 54, 62, 64, 72, 73, 74],
                },
                Octopus {
                    value: 8,
                    neighbours: [53, 54, 55, 63, 65, 73, 74, 75],
                },
                Octopus {
                    value: 4,
                    neighbours: [54, 55, 56, 64, 66, 74, 75, 76],
                },
                Octopus {
                    value: 1,
                    neighbours: [55, 56, 57, 65, 67, 75, 76, 77],
                },
                Octopus {
                    value: 7,
                    neighbours: [56, 57, 58, 66, 68, 76, 77, 78],
                },
                Octopus {
                    value: 2,
                    neighbours: [57, 58, 59, 67, 69, 77, 78, 79],
                },
                Octopus {
                    value: 1,
                    neighbours: [58, 59, 100, 68, 100, 78, 79, 100],
                },
                // 6882881134
                Octopus {
                    value: 6,
                    neighbours: [100, 60, 61, 100, 71, 100, 80, 81],
                },
                Octopus {
                    value: 8,
                    neighbours: [60, 61, 62, 70, 72, 80, 81, 82],
                },
                Octopus {
                    value: 8,
                    neighbours: [61, 62, 63, 71, 73, 81, 82, 83],
                },
                Octopus {
                    value: 2,
                    neighbours: [62, 63, 64, 72, 74, 82, 83, 84],
                },
                Octopus {
                    value: 8,
                    neighbours: [63, 64, 65, 73, 75, 83, 84, 85],
                },
                Octopus {
                    value: 8,
                    neighbours: [64, 65, 66, 74, 76, 84, 85, 86],
                },
                Octopus {
                    value: 1,
                    neighbours: [65, 66, 67, 75, 77, 85, 86, 87],
                },
                Octopus {
                    value: 1,
                    neighbours: [66, 67, 68, 76, 78, 86, 87, 88],
                },
                Octopus {
                    value: 3,
                    neighbours: [67, 68, 69, 77, 79, 87, 88, 89],
                },
                Octopus {
                    value: 4,
                    neighbours: [68, 69, 100, 78, 100, 88, 89, 100],
                },
                // 4846848554
                Octopus {
                    value: 4,
                    neighbours: [100, 70, 71, 100, 81, 100, 90, 91],
                },
                Octopus {
                    value: 8,
                    neighbours: [70, 71, 72, 80, 82, 90, 91, 92],
                },
                Octopus {
                    value: 4,
                    neighbours: [71, 72, 73, 81, 83, 91, 92, 93],
                },
                Octopus {
                    value: 6,
                    neighbours: [72, 73, 74, 82, 84, 92, 93, 94],
                },
                Octopus {
                    value: 8,
                    neighbours: [73, 74, 75, 83, 85, 93, 94, 95],
                },
                Octopus {
                    value: 4,
                    neighbours: [74, 75, 76, 84, 86, 94, 95, 96],
                },
                Octopus {
                    value: 8,
                    neighbours: [75, 76, 77, 85, 87, 95, 96, 97],
                },
                Octopus {
                    value: 5,
                    neighbours: [76, 77, 78, 86, 88, 96, 97, 98],
                },
                Octopus {
                    value: 5,
                    neighbours: [77, 78, 79, 87, 89, 97, 98, 99],
                },
                Octopus {
                    value: 4,
                    neighbours: [78, 79, 100, 88, 100, 98, 99, 100],
                },
                // 5283751526
                Octopus {
                    value: 5,
                    neighbours: [100, 80, 81, 100, 91, 100, 100, 100],
                },
                Octopus {
                    value: 2,
                    neighbours: [80, 81, 82, 90, 92, 100, 100, 100],
                },
                Octopus {
                    value: 8,
                    neighbours: [81, 82, 83, 91, 93, 100, 100, 100],
                },
                Octopus {
                    value: 3,
                    neighbours: [82, 83, 84, 92, 94, 100, 100, 100],
                },
                Octopus {
                    value: 7,
                    neighbours: [83, 84, 85, 93, 95, 100, 100, 100],
                },
                Octopus {
                    value: 5,
                    neighbours: [84, 85, 86, 94, 96, 100, 100, 100],
                },
                Octopus {
                    value: 1,
                    neighbours: [85, 86, 87, 95, 97, 100, 100, 100],
                },
                Octopus {
                    value: 5,
                    neighbours: [86, 87, 88, 96, 98, 100, 100, 100],
                },
                Octopus {
                    value: 2,
                    neighbours: [87, 88, 89, 97, 99, 100, 100, 100],
                },
                Octopus {
                    value: 6,
                    neighbours: [88, 89, 100, 98, 100, 100, 100, 100],
                },
            ],
        }
    }
}
