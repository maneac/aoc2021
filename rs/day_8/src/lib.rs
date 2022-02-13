#![deny(clippy::all)]
#![feature(test)]
extern crate test;

use std::{fs::read_to_string, path::Path, str::FromStr};

pub type Input = Vec<Display>;

pub const PART_1: usize = 284;
pub const PART_2: usize = 973499;

pub fn read_data(data_dir: &str) -> Input {
    let contents = read_to_string(Path::new(data_dir).join("day_8.txt")).unwrap();

    parse_contents(contents.trim())
}

fn parse_contents(contents: &str) -> Input {
    contents
        .lines()
        .map(|line| Display::from_str(line).unwrap())
        .collect()
}

pub fn part_1(input: &Input) -> usize {
    input.iter().fold(0usize, |acc, display| {
        acc + display
            .output
            .iter()
            .filter(|out| matches!(out.count_ones(), 2 | 3 | 4 | 7))
            .count()
    })
}

pub fn part_2(input: &Input) -> usize {
    input.iter().map(|d| d.decode()).sum()
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Display {
    digits: [u8; 10],
    output: [u8; 4],
}

impl FromStr for Display {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (displays, output_values) = s.split_once(" | ").unwrap();

        let digits = displays.split_ascii_whitespace().enumerate().fold(
            [0u8; 10],
            |mut acc, (idx, digit)| {
                acc[idx] = digit
                    .chars()
                    .fold(0u8, |acc, c| acc | (1 << (c as u8 - b'a')));
                acc
            },
        );

        let output = output_values.split_ascii_whitespace().enumerate().fold(
            [0u8; 4],
            |mut acc, (idx, digit)| {
                acc[idx] = digit
                    .chars()
                    .fold(0u8, |acc, c| acc | (1 << (c as u8 - b'a')));
                acc
            },
        );

        Ok(Self { digits, output })
    }
}

impl Display {
    const NUMS: [u8; 10] = [
        0b1110111, // 0
        0b0100100, // 1
        0b1011101, // 2
        0b1101101, // 3
        0b0101110, // 4
        0b1101011, // 5
        0b1111011, // 6
        0b0100101, // 7
        0b1111111, // 8
        0b1101111, // 9
    ];

    fn decode(&self) -> usize {
        let mut mapping = [0u8; 7];

        let one = self.digits.iter().find(|d| d.count_ones() == 2).unwrap();
        let seven = self.digits.iter().find(|d| d.count_ones() == 3).unwrap();
        let four = self.digits.iter().find(|d| d.count_ones() == 4).unwrap();

        // &!: bitwise difference
        // 1: __c__f_
        // 7: a_c__f_

        // a = 7 - 1
        mapping[0] = seven & !one;

        // 5s:
        // 2: a_cde_g
        // 3: a_cd_fg
        // 5: ab_d_fg
        let len_fives = self
            .digits
            .iter()
            .filter_map(|&d| if d.count_ones() == 5 { Some(d) } else { None })
            .collect::<Vec<u8>>();

        // 3 contains 1 => 3 identified
        let three = len_fives.iter().find(|&d| (d & one).eq(one)).unwrap();

        // 3: a_cd_fg
        // 4: _bcd_f_
        // (4 n 3): __cd_f_

        // b = 4 - (4 n 3)
        mapping[1] = four & !(four & three);

        // g = 3 - (4 n 3) - a
        mapping[6] = three & !(four & three) & !mapping[0];

        // 5 contains b => 5 and 2 identified
        let five = len_fives.iter().find(|&d| d & mapping[1] > 0).unwrap();
        let two = len_fives
            .iter()
            .find(|&d| d.ne(five) && d.ne(three))
            .unwrap();

        // (3 n 5): a__d_fg
        // (2 n 3): a_cd__g

        // c = 3 - (3 n 5)
        mapping[2] = three & !(three & five);

        // e = 2 - (2 n 3)
        mapping[4] = two & !(two & three);

        // f = 3 - (2 n 3)
        mapping[5] = three & !(two & three);

        // d = remainder
        mapping[3] = mapping.iter().fold(0b1111111, |acc, n| acc ^ n);

        self.output
            .iter()
            .enumerate()
            .fold(0usize, |acc, (pos, v)| {
                let num_as_bits = mapping.iter().enumerate().fold(0u8, |acc, (idx, num)| {
                    if v & num > 0 {
                        acc | (1 << idx)
                    } else {
                        acc
                    }
                });
                let num = Display::NUMS
                    .iter()
                    .position(|&v| v == num_as_bits)
                    .unwrap();
                acc + (num * 10usize.pow((4 - pos - 1) as u32))
            })
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
        fn example_1() {
            run(&Case{
                input:            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
                expected: example_1_data(),
            })
        }

        #[test]
        fn example_2() {
            run(&Case {
                input: "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
",
                expected: example_2_data(),
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
        fn example_1() {
            run(&Case {
                data: example_1_data(),
                expected: 0,
            })
        }

        #[test]
        fn example_2() {
            run(&Case {
                data: example_2_data(),
                expected: 26,
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
        fn example_1() {
            run(&Case {
                data: example_1_data(),
                expected: 5353,
            })
        }

        #[test]
        fn example_2() {
            run(&Case {
                data: example_2_data(),
                expected: 61229,
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

    fn bit(chr: char) -> u8 {
        1 << (chr as u8 - b'a')
    }

    fn example_1_data() -> Input {
        vec![Display {
            digits: [
                bit('a') | bit('c') | bit('e') | bit('d') | bit('g') | bit('f') | bit('b'),
                bit('c') | bit('d') | bit('f') | bit('b') | bit('e'),
                bit('g') | bit('c') | bit('d') | bit('f') | bit('a'),
                bit('f') | bit('b') | bit('c') | bit('a') | bit('d'),
                bit('d') | bit('a') | bit('b'),
                bit('c') | bit('e') | bit('f') | bit('a') | bit('b') | bit('d'),
                bit('c') | bit('d') | bit('f') | bit('g') | bit('e') | bit('b'),
                bit('e') | bit('a') | bit('f') | bit('b'),
                bit('c') | bit('a') | bit('g') | bit('e') | bit('d') | bit('b'),
                bit('a') | bit('b'),
            ],
            output: [
                bit('c') | bit('d') | bit('f') | bit('e') | bit('b'),
                bit('f') | bit('c') | bit('a') | bit('d') | bit('b'),
                bit('c') | bit('d') | bit('f') | bit('e') | bit('b'),
                bit('c') | bit('d') | bit('b') | bit('a') | bit('f'),
            ],
        }]
    }

    fn example_2_data() -> Input {
        vec![
            Display {
                digits: [
                    bit('b') | bit('e'),
                    bit('c') | bit('f') | bit('b') | bit('e') | bit('g') | bit('a') | bit('d'),
                    bit('c') | bit('b') | bit('d') | bit('g') | bit('e') | bit('f'),
                    bit('f') | bit('g') | bit('a') | bit('e') | bit('c') | bit('d'),
                    bit('c') | bit('g') | bit('e') | bit('b'),
                    bit('f') | bit('d') | bit('c') | bit('g') | bit('e'),
                    bit('a') | bit('g') | bit('e') | bit('b') | bit('f') | bit('d'),
                    bit('f') | bit('e') | bit('c') | bit('d') | bit('b'),
                    bit('f') | bit('a') | bit('b') | bit('c') | bit('d'),
                    bit('e') | bit('d') | bit('b'),
                ],
                output: [
                    bit('f') | bit('d') | bit('g') | bit('a') | bit('c') | bit('b') | bit('e'),
                    bit('c') | bit('e') | bit('f') | bit('d') | bit('b'),
                    bit('c') | bit('e') | bit('f') | bit('b') | bit('g') | bit('d'),
                    bit('g') | bit('c') | bit('b') | bit('e'),
                ],
            },
            Display {
                digits: [
                    bit('e') | bit('d') | bit('b') | bit('f') | bit('g') | bit('a'),
                    bit('b') | bit('e') | bit('g') | bit('c') | bit('d'),
                    bit('c') | bit('b') | bit('g'),
                    bit('g') | bit('c'),
                    bit('g') | bit('c') | bit('a') | bit('d') | bit('e') | bit('b') | bit('f'),
                    bit('f') | bit('b') | bit('g') | bit('d') | bit('e'),
                    bit('a') | bit('c') | bit('b') | bit('g') | bit('f') | bit('d'),
                    bit('a') | bit('b') | bit('c') | bit('d') | bit('e'),
                    bit('g') | bit('f') | bit('c') | bit('b') | bit('e') | bit('d'),
                    bit('g') | bit('f') | bit('e') | bit('c'),
                ],
                output: [
                    bit('f') | bit('c') | bit('g') | bit('e') | bit('d') | bit('b'),
                    bit('c') | bit('g') | bit('b'),
                    bit('d') | bit('g') | bit('e') | bit('b') | bit('a') | bit('c') | bit('f'),
                    bit('g') | bit('c'),
                ],
            },
            Display {
                digits: [
                    bit('f') | bit('g') | bit('a') | bit('e') | bit('b') | bit('d'),
                    bit('c') | bit('g'),
                    bit('b') | bit('d') | bit('a') | bit('e') | bit('c'),
                    bit('g') | bit('d') | bit('a') | bit('f') | bit('b'),
                    bit('a') | bit('g') | bit('b') | bit('c') | bit('f') | bit('d'),
                    bit('g') | bit('d') | bit('c') | bit('b') | bit('e') | bit('f'),
                    bit('b') | bit('g') | bit('c') | bit('a') | bit('d'),
                    bit('g') | bit('f') | bit('a') | bit('c'),
                    bit('g') | bit('c') | bit('b'),
                    bit('c') | bit('d') | bit('g') | bit('a') | bit('b') | bit('e') | bit('f'),
                ],
                output: [
                    bit('c') | bit('g'),
                    bit('c') | bit('g'),
                    bit('f') | bit('d') | bit('c') | bit('a') | bit('g') | bit('b'),
                    bit('c') | bit('b') | bit('g'),
                ],
            },
            Display {
                digits: [
                    bit('f') | bit('b') | bit('e') | bit('g') | bit('c') | bit('d'),
                    bit('c') | bit('b') | bit('d'),
                    bit('a') | bit('d') | bit('c') | bit('e') | bit('f') | bit('b'),
                    bit('d') | bit('a') | bit('g') | bit('e') | bit('b'),
                    bit('a') | bit('f') | bit('c') | bit('b'),
                    bit('b') | bit('c'),
                    bit('a') | bit('e') | bit('f') | bit('d') | bit('c'),
                    bit('e') | bit('c') | bit('d') | bit('a') | bit('b'),
                    bit('f') | bit('g') | bit('d') | bit('e') | bit('c') | bit('a'),
                    bit('f') | bit('c') | bit('d') | bit('b') | bit('e') | bit('g') | bit('a'),
                ],
                output: [
                    bit('e') | bit('f') | bit('a') | bit('b') | bit('c') | bit('d'),
                    bit('c') | bit('e') | bit('d') | bit('b') | bit('a'),
                    bit('g') | bit('a') | bit('d') | bit('f') | bit('e') | bit('c'),
                    bit('c') | bit('b'),
                ],
            },
            Display {
                digits: [
                    bit('a') | bit('e') | bit('c') | bit('b') | bit('f') | bit('d') | bit('g'),
                    bit('f') | bit('b') | bit('g'),
                    bit('g') | bit('f'),
                    bit('b') | bit('a') | bit('f') | bit('e') | bit('g'),
                    bit('d') | bit('b') | bit('e') | bit('f') | bit('a'),
                    bit('f') | bit('c') | bit('g') | bit('e'),
                    bit('g') | bit('c') | bit('b') | bit('e') | bit('a'),
                    bit('f') | bit('c') | bit('a') | bit('e') | bit('g') | bit('b'),
                    bit('d') | bit('g') | bit('c') | bit('e') | bit('a') | bit('b'),
                    bit('f') | bit('c') | bit('b') | bit('d') | bit('g') | bit('a'),
                ],
                output: [
                    bit('g') | bit('e') | bit('c') | bit('f'),
                    bit('e') | bit('g') | bit('d') | bit('c') | bit('a') | bit('b') | bit('f'),
                    bit('b') | bit('g') | bit('f'),
                    bit('b') | bit('f') | bit('g') | bit('e') | bit('a'),
                ],
            },
            Display {
                digits: [
                    bit('f') | bit('g') | bit('e') | bit('a') | bit('b'),
                    bit('c') | bit('a'),
                    bit('a') | bit('f') | bit('c') | bit('e') | bit('b') | bit('g'),
                    bit('b') | bit('d') | bit('a') | bit('c') | bit('f') | bit('e') | bit('g'),
                    bit('c') | bit('f') | bit('a') | bit('e') | bit('d') | bit('g'),
                    bit('g') | bit('c') | bit('f') | bit('d') | bit('b'),
                    bit('b') | bit('a') | bit('e') | bit('c'),
                    bit('b') | bit('f') | bit('a') | bit('d') | bit('e') | bit('g'),
                    bit('b') | bit('a') | bit('f') | bit('g') | bit('c'),
                    bit('a') | bit('c') | bit('f'),
                ],
                output: [
                    bit('g') | bit('e') | bit('b') | bit('d') | bit('c') | bit('f') | bit('a'),
                    bit('e') | bit('c') | bit('b') | bit('a'),
                    bit('c') | bit('a'),
                    bit('f') | bit('a') | bit('d') | bit('e') | bit('g') | bit('c') | bit('b'),
                ],
            },
            Display {
                digits: [
                    bit('d') | bit('b') | bit('c') | bit('f') | bit('g'),
                    bit('f') | bit('g') | bit('d'),
                    bit('b') | bit('d') | bit('e') | bit('g') | bit('c') | bit('a') | bit('f'),
                    bit('f') | bit('g') | bit('e') | bit('c'),
                    bit('a') | bit('e') | bit('g') | bit('b') | bit('d') | bit('f'),
                    bit('e') | bit('c') | bit('d') | bit('f') | bit('a') | bit('b'),
                    bit('f') | bit('b') | bit('e') | bit('d') | bit('c'),
                    bit('d') | bit('a') | bit('c') | bit('g') | bit('b'),
                    bit('g') | bit('d') | bit('c') | bit('e') | bit('b') | bit('f'),
                    bit('g') | bit('f'),
                ],
                output: [
                    bit('c') | bit('e') | bit('f') | bit('g'),
                    bit('d') | bit('c') | bit('b') | bit('e') | bit('f'),
                    bit('f') | bit('c') | bit('g') | bit('e'),
                    bit('g') | bit('b') | bit('c') | bit('a') | bit('d') | bit('f') | bit('e'),
                ],
            },
            Display {
                digits: [
                    bit('b') | bit('d') | bit('f') | bit('e') | bit('g') | bit('c'),
                    bit('c') | bit('b') | bit('e') | bit('g') | bit('a') | bit('f'),
                    bit('g') | bit('e') | bit('c') | bit('b') | bit('f'),
                    bit('d') | bit('f') | bit('c') | bit('a') | bit('g') | bit('e'),
                    bit('b') | bit('d') | bit('a') | bit('c') | bit('g'),
                    bit('e') | bit('d'),
                    bit('b') | bit('e') | bit('d') | bit('f'),
                    bit('c') | bit('e') | bit('d'),
                    bit('a') | bit('d') | bit('c') | bit('b') | bit('e') | bit('f') | bit('g'),
                    bit('g') | bit('e') | bit('b') | bit('c') | bit('d'),
                ],
                output: [
                    bit('e') | bit('d'),
                    bit('b') | bit('c') | bit('g') | bit('a') | bit('f') | bit('e'),
                    bit('c') | bit('d') | bit('g') | bit('b') | bit('a'),
                    bit('c') | bit('b') | bit('g') | bit('e') | bit('f'),
                ],
            },
            Display {
                digits: [
                    bit('e') | bit('g') | bit('a') | bit('d') | bit('f') | bit('b'),
                    bit('c') | bit('d') | bit('b') | bit('f') | bit('e') | bit('g'),
                    bit('c') | bit('e') | bit('g') | bit('d'),
                    bit('f') | bit('e') | bit('c') | bit('a') | bit('b'),
                    bit('c') | bit('g') | bit('b'),
                    bit('g') | bit('b') | bit('d') | bit('e') | bit('f') | bit('c') | bit('a'),
                    bit('c') | bit('g'),
                    bit('f') | bit('g') | bit('c') | bit('d') | bit('a') | bit('b'),
                    bit('e') | bit('g') | bit('f') | bit('d') | bit('b'),
                    bit('b') | bit('f') | bit('c') | bit('e') | bit('g'),
                ],
                output: [
                    bit('g') | bit('b') | bit('d') | bit('f') | bit('c') | bit('a') | bit('e'),
                    bit('b') | bit('g') | bit('c'),
                    bit('c') | bit('g'),
                    bit('c') | bit('g') | bit('b'),
                ],
            },
            Display {
                digits: [
                    bit('g') | bit('c') | bit('a') | bit('f') | bit('b'),
                    bit('g') | bit('c') | bit('f'),
                    bit('d') | bit('c') | bit('a') | bit('e') | bit('b') | bit('f') | bit('g'),
                    bit('e') | bit('c') | bit('a') | bit('g') | bit('b'),
                    bit('g') | bit('f'),
                    bit('a') | bit('b') | bit('c') | bit('d') | bit('e') | bit('g'),
                    bit('g') | bit('a') | bit('e') | bit('f'),
                    bit('c') | bit('a') | bit('f') | bit('b') | bit('g') | bit('e'),
                    bit('f') | bit('d') | bit('b') | bit('a') | bit('c'),
                    bit('f') | bit('e') | bit('g') | bit('b') | bit('d') | bit('c'),
                ],
                output: [
                    bit('f') | bit('g') | bit('a') | bit('e'),
                    bit('c') | bit('f') | bit('g') | bit('a') | bit('b'),
                    bit('f') | bit('g'),
                    bit('b') | bit('a') | bit('g') | bit('c') | bit('e'),
                ],
            },
        ]
    }
}
