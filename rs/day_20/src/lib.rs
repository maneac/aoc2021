#![deny(clippy::all)]
#![feature(test)]
extern crate test;

use std::{cmp::Ordering, fs::read_to_string, hash::Hash, path::Path};

pub type Input = ImageProcessor;

pub const PART_1: usize = 5359;
pub const PART_2: usize = 12333;

pub fn read_data(data_dir: &str) -> Input {
    let contents = read_to_string(Path::new(data_dir).join("day_20.txt")).unwrap();

    parse_contents(contents.trim())
}

fn parse_contents(contents: &str) -> Input {
    let (algo, img) = contents.split_once("\n\n").unwrap();

    let mut algorithm = [false; 512];
    for (idx, chr) in algo.char_indices() {
        algorithm[idx] = chr == '#';
    }

    let image = img.lines().fold(Vec::new(), |mut acc, line| {
        acc.push(line.chars().fold(Vec::new(), |mut acc, chr| {
            acc.push(chr == '#');
            acc
        }));
        acc
    });

    Input {
        algorithm,
        image,
        default: false,
    }
}

pub fn part_1(input: &Input) -> usize {
    let mut image = input.clone();

    for _ in 0..2 {
        image.enhance();
    }

    image.pixel_count()
}

pub fn part_2(input: &Input) -> usize {
    let mut image = input.clone();

    for _ in 0..50 {
        image.enhance();
    }

    image.pixel_count()
}

#[derive(Debug, Eq, PartialOrd, Ord, Clone, Copy, Default)]
struct Pixel {
    x: isize,
    y: isize,
}

impl Hash for Pixel {
    fn hash<H>(&self, h: &mut H)
    where
        H: std::hash::Hasher,
    {
        h.write_u128((self.x as u128) << 64 | self.y as u128)
    }
}

impl PartialEq for Pixel {
    fn eq(&self, rhs: &Self) -> bool {
        self.x.cmp(&rhs.x).then(self.y.cmp(&rhs.y)) == Ordering::Equal
    }
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct ImageProcessor {
    algorithm: [bool; 512],
    image: Vec<Vec<bool>>,
    default: bool,
}

impl Default for ImageProcessor {
    fn default() -> Self {
        Self {
            algorithm: [false; 512],
            image: Vec::default(),
            default: false,
        }
    }
}

impl ImageProcessor {
    fn enhance(&mut self) {
        let row_len = self.image[0].len() + 6;
        let empty_row = vec![self.default; row_len];
        let mut image = Vec::with_capacity(self.image.len() + 6);

        // extend by 3 in all directions
        image.push(empty_row.clone());
        image.push(empty_row.clone());
        image.push(empty_row.clone());
        for row in &mut self.image {
            let mut new_row = Vec::with_capacity(row_len);
            new_row.push(self.default);
            new_row.push(self.default);
            new_row.push(self.default);
            new_row.append(row);
            new_row.push(self.default);
            new_row.push(self.default);
            new_row.push(self.default);
            image.push(new_row);
        }
        image.push(empty_row.clone());
        image.push(empty_row.clone());
        image.push(empty_row);

        let mut new_image = Vec::with_capacity(image.len() - 2);
        for c_y in 1..(image.len() - 1) {
            let mut new_row = Vec::with_capacity(row_len - 2);
            for c_x in 1..(row_len - 1) {
                let mut idx = 0;
                for row in image.iter().skip(c_y - 1).take(3) {
                    for &is_lit in row.iter().skip(c_x - 1).take(3) {
                        idx <<= 1;
                        if is_lit {
                            idx |= 1;
                        }
                    }
                }
                new_row.push(self.algorithm[idx]);
            }
            new_image.push(new_row);
        }

        self.image = new_image;
        self.default = if self.default {
            self.algorithm[511]
        } else {
            self.algorithm[0]
        };
    }

    fn pixel_count(&self) -> usize {
        self.image.iter().flatten().filter(|&&px| px).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    mod total {
        use super::*;

        #[bench]
        fn actual(b: &mut Bencher) {
            b.iter(|| {
                let data = read_data("../../data");
                assert_ne!(data, Input::default());
                assert_eq!(PART_1, part_1(&data));
                assert_eq!(PART_2, part_2(&data));
            })
        }
    }

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
                input: "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###",
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
                expected: 35,
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
                expected: 3351,
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
            algorithm: [
                false, false, true, false, true, false, false, true, true, true, true, true, false,
                true, false, true, false, true, false, true, true, true, false, true, true, false,
                false, false, false, false, true, true, true, false, true, true, false, true,
                false, false, true, true, true, false, true, true, true, true, false, false, true,
                true, true, true, true, false, false, true, false, false, false, false, true,
                false, false, true, false, false, true, true, false, false, true, true, true,
                false, false, true, true, true, true, true, true, false, true, true, true, false,
                false, false, true, true, true, true, false, false, true, false, false, true, true,
                true, true, true, false, false, true, true, false, false, true, false, true, true,
                true, true, true, false, false, false, true, true, false, true, false, true, false,
                false, true, false, true, true, false, false, true, false, true, false, false,
                false, false, false, false, true, false, true, true, true, false, true, true, true,
                true, true, true, false, true, true, true, false, true, true, true, true, false,
                false, false, true, false, true, true, false, true, true, false, false, true,
                false, false, true, false, false, true, true, true, true, true, false, false,
                false, false, false, true, false, true, false, false, false, false, true, true,
                true, false, false, true, false, true, true, false, false, false, false, false,
                false, true, false, false, false, false, false, true, false, false, true, false,
                false, true, false, false, true, true, false, false, true, false, false, false,
                true, true, false, true, true, true, true, true, true, false, true, true, true,
                true, false, true, true, true, true, false, true, false, true, false, false, false,
                true, false, false, false, false, false, false, false, true, false, false, true,
                false, true, false, true, false, false, false, true, true, true, true, false, true,
                true, false, true, false, false, false, false, false, false, true, false, false,
                true, false, false, false, true, true, false, true, false, true, true, false,
                false, true, false, false, false, true, true, false, true, false, true, true,
                false, false, true, true, true, false, true, false, false, false, false, false,
                false, true, false, true, false, false, false, false, false, false, false, true,
                false, true, false, true, false, true, true, true, true, false, true, true, true,
                false, true, true, false, false, false, true, false, false, false, false, false,
                true, true, true, true, false, true, false, false, true, false, false, true, false,
                true, true, false, true, false, false, false, false, true, true, false, false,
                true, false, true, true, true, true, false, false, false, false, true, true, false,
                false, false, true, true, false, false, true, false, false, false, true, false,
                false, false, false, false, false, true, false, true, false, false, false, false,
                false, false, false, true, false, false, false, false, false, false, false, true,
                true, false, false, true, true, true, true, false, false, true, false, false,
                false, true, false, true, false, true, false, false, false, true, true, false,
                false, true, false, true, false, false, true, true, true, false, false, true, true,
                true, true, true, false, false, false, false, false, false, false, false, true,
                false, false, true, true, true, true, false, false, false, false, false, false,
                true, false, false, true,
            ],
            image: vec![
                vec![true, false, false, true, false],
                vec![true, false, false, false, false],
                vec![true, true, false, false, true],
                vec![false, false, true, false, false],
                vec![false, false, true, true, true],
            ],
            default: false,
        }
    }
}
