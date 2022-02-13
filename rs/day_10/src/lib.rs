#![deny(clippy::all)]
#![feature(test)]
extern crate test;

use std::{fs::read_to_string, path::Path, slice::Iter};

pub type Input = Vec<Vec<Char>>;

pub const PART_1: usize = 266301;
pub const PART_2: usize = 3404870164;

pub fn read_data(data_dir: &str) -> Input {
    let contents = read_to_string(Path::new(data_dir).join("day_10.txt")).unwrap();

    parse_contents(contents.trim())
}

fn parse_contents(contents: &str) -> Input {
    contents
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '(' => Char::L(Bracket::Parenthesis),
                    '[' => Char::L(Bracket::Square),
                    '{' => Char::L(Bracket::Curly),
                    '<' => Char::L(Bracket::Angle),
                    ')' => Char::R(Bracket::Parenthesis),
                    ']' => Char::R(Bracket::Square),
                    '}' => Char::R(Bracket::Curly),
                    '>' => Char::R(Bracket::Angle),
                    _ => panic!("invalid character in input: {}", c),
                })
                .collect()
        })
        .collect()
}

pub fn part_1(input: &Input) -> usize {
    input.iter().fold(0usize, |acc, line| {
        if let Err(Code::Illegal(bracket)) = validate_line(&mut line.iter(), None) {
            match bracket {
                Bracket::Parenthesis => acc + 3,
                Bracket::Square => acc + 57,
                Bracket::Curly => acc + 1197,
                Bracket::Angle => acc + 25137,
            }
        } else {
            acc
        }
    })
}

pub fn part_2(input: &Input) -> usize {
    let mut scores = input.iter().fold(Vec::new(), |mut acc, line| {
        let score = match fix_line(&mut line.iter(), None) {
            Ok(fix) => fix.iter().fold(0usize, |acc, &c| {
                acc * 5
                    + match c {
                        Bracket::Parenthesis => 1,
                        Bracket::Square => 2,
                        Bracket::Curly => 3,
                        Bracket::Angle => 4,
                    }
            }),
            _ => 0,
        };
        if score > 0 {
            acc.push(score);
        }
        acc
    });

    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Code {
    Incomplete,
    Illegal(Bracket),
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Char {
    L(Bracket),
    R(Bracket),
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Bracket {
    Parenthesis,
    Square,
    Curly,
    Angle,
}

fn validate_line(line: &mut Iter<Char>, prev: Option<Bracket>) -> Result<(), Code> {
    loop {
        let c = match line.next() {
            None => return Err(Code::Incomplete),
            Some(&c) => c,
        };
        match c {
            Char::L(b) => {
                if let Err(code) = validate_line(line, Some(b)) {
                    return Err(code);
                }
            }
            Char::R(Bracket::Parenthesis) if prev == Some(Bracket::Parenthesis) => return Ok(()),
            Char::R(Bracket::Square) if prev == Some(Bracket::Square) => return Ok(()),
            Char::R(Bracket::Curly) if prev == Some(Bracket::Curly) => return Ok(()),
            Char::R(Bracket::Angle) if prev == Some(Bracket::Angle) => return Ok(()),
            Char::R(b) => return Err(Code::Illegal(b)),
        }
    }
}

fn fix_line(line: &mut Iter<Char>, prev: Option<Bracket>) -> Result<Vec<Bracket>, Code> {
    let mut output = vec![];
    loop {
        let c = match line.next() {
            Some(&c) => c,
            None => match prev {
                Some(b) => {
                    output.push(b);
                    return Ok(output);
                }
                None => return Ok(output),
            },
        };

        match c {
            Char::L(b) => match fix_line(line, Some(b)) {
                Ok(mut chars) => {
                    if !chars.is_empty() {
                        output.append(&mut chars);
                    }
                }
                Err(e) => return Err(e),
            },
            Char::R(Bracket::Parenthesis) if prev == Some(Bracket::Parenthesis) => {
                return Ok(output)
            }
            Char::R(Bracket::Square) if prev == Some(Bracket::Square) => return Ok(output),
            Char::R(Bracket::Curly) if prev == Some(Bracket::Curly) => return Ok(output),
            Char::R(Bracket::Angle) if prev == Some(Bracket::Angle) => return Ok(output),
            Char::R(b) => return Err(Code::Illegal(b)),
        }
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
                input: "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]",
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
                expected: 26397,
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
                expected: 288957,
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
        vec![
            vec![
                Char::L(Bracket::Square),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Angle),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Parenthesis),
                Char::R(Bracket::Parenthesis),
                Char::R(Bracket::Parenthesis),
                Char::L(Bracket::Square),
                Char::R(Bracket::Square),
                Char::R(Bracket::Angle),
                Char::L(Bracket::Square),
                Char::L(Bracket::Square),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Square),
                Char::R(Bracket::Square),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Angle),
                Char::L(Bracket::Parenthesis),
                Char::R(Bracket::Parenthesis),
                Char::L(Bracket::Angle),
                Char::R(Bracket::Angle),
                Char::R(Bracket::Angle),
            ],
            vec![
                Char::L(Bracket::Square),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Parenthesis),
                Char::R(Bracket::Parenthesis),
                Char::L(Bracket::Square),
                Char::L(Bracket::Angle),
                Char::R(Bracket::Angle),
                Char::R(Bracket::Square),
                Char::R(Bracket::Parenthesis),
                Char::R(Bracket::Square),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Square),
                Char::L(Bracket::Angle),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Angle),
                Char::L(Bracket::Angle),
                Char::L(Bracket::Square),
                Char::R(Bracket::Square),
                Char::R(Bracket::Angle),
                Char::R(Bracket::Angle),
                Char::L(Bracket::Parenthesis),
            ],
            vec![
                Char::L(Bracket::Curly),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Square),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Angle),
                Char::L(Bracket::Curly),
                Char::R(Bracket::Curly),
                Char::L(Bracket::Square),
                Char::L(Bracket::Angle),
                Char::R(Bracket::Angle),
                Char::L(Bracket::Square),
                Char::R(Bracket::Square),
                Char::R(Bracket::Curly),
                Char::R(Bracket::Angle),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Square),
                Char::R(Bracket::Square),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Square),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Angle),
                Char::L(Bracket::Parenthesis),
                Char::R(Bracket::Parenthesis),
                Char::R(Bracket::Angle),
            ],
            vec![
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Angle),
                Char::R(Bracket::Angle),
                Char::R(Bracket::Curly),
                Char::L(Bracket::Angle),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Angle),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Angle),
                Char::R(Bracket::Angle),
                Char::R(Bracket::Curly),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Square),
                Char::R(Bracket::Square),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Square),
                Char::R(Bracket::Square),
                Char::L(Bracket::Curly),
                Char::R(Bracket::Curly),
            ],
            vec![
                Char::L(Bracket::Square),
                Char::L(Bracket::Square),
                Char::L(Bracket::Angle),
                Char::L(Bracket::Square),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Square),
                Char::R(Bracket::Square),
                Char::R(Bracket::Parenthesis),
                Char::R(Bracket::Parenthesis),
                Char::L(Bracket::Angle),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Square),
                Char::L(Bracket::Square),
                Char::L(Bracket::Curly),
                Char::R(Bracket::Curly),
                Char::L(Bracket::Square),
                Char::L(Bracket::Square),
                Char::L(Bracket::Parenthesis),
                Char::R(Bracket::Parenthesis),
                Char::R(Bracket::Square),
                Char::R(Bracket::Square),
                Char::R(Bracket::Square),
            ],
            vec![
                Char::L(Bracket::Square),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Square),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Curly),
                Char::R(Bracket::Curly),
                Char::R(Bracket::Square),
                Char::L(Bracket::Curly),
                Char::R(Bracket::Curly),
                Char::R(Bracket::Curly),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Square),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Square),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Curly),
                Char::R(Bracket::Curly),
                Char::R(Bracket::Curly),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Square),
                Char::R(Bracket::Square),
            ],
            vec![
                Char::L(Bracket::Curly),
                Char::L(Bracket::Angle),
                Char::L(Bracket::Square),
                Char::L(Bracket::Square),
                Char::R(Bracket::Square),
                Char::R(Bracket::Square),
                Char::R(Bracket::Angle),
                Char::R(Bracket::Curly),
                Char::L(Bracket::Angle),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Square),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Square),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Square),
                Char::R(Bracket::Square),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Parenthesis),
                Char::R(Bracket::Parenthesis),
                Char::L(Bracket::Square),
                Char::L(Bracket::Square),
                Char::L(Bracket::Square),
                Char::R(Bracket::Square),
            ],
            vec![
                Char::L(Bracket::Square),
                Char::L(Bracket::Angle),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Angle),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Angle),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Angle),
                Char::L(Bracket::Curly),
                Char::R(Bracket::Curly),
                Char::R(Bracket::Parenthesis),
                Char::R(Bracket::Parenthesis),
                Char::R(Bracket::Angle),
                Char::L(Bracket::Angle),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Square),
                Char::R(Bracket::Square),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Square),
                Char::R(Bracket::Square),
                Char::L(Bracket::Parenthesis),
                Char::R(Bracket::Parenthesis),
            ],
            vec![
                Char::L(Bracket::Angle),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Square),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Square),
                Char::L(Bracket::Square),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Angle),
                Char::R(Bracket::Angle),
                Char::L(Bracket::Parenthesis),
                Char::R(Bracket::Parenthesis),
                Char::R(Bracket::Parenthesis),
                Char::L(Bracket::Curly),
                Char::R(Bracket::Curly),
                Char::R(Bracket::Square),
                Char::R(Bracket::Angle),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Angle),
                Char::L(Bracket::Angle),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Curly),
            ],
            vec![
                Char::L(Bracket::Angle),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Parenthesis),
                Char::L(Bracket::Square),
                Char::L(Bracket::Curly),
                Char::L(Bracket::Curly),
                Char::R(Bracket::Curly),
                Char::R(Bracket::Curly),
                Char::L(Bracket::Square),
                Char::L(Bracket::Angle),
                Char::L(Bracket::Square),
                Char::L(Bracket::Square),
                Char::L(Bracket::Square),
                Char::L(Bracket::Angle),
                Char::R(Bracket::Angle),
                Char::L(Bracket::Curly),
                Char::R(Bracket::Curly),
                Char::R(Bracket::Square),
                Char::R(Bracket::Square),
                Char::R(Bracket::Square),
                Char::R(Bracket::Angle),
                Char::L(Bracket::Square),
                Char::R(Bracket::Square),
                Char::R(Bracket::Square),
            ],
        ]
    }
}
