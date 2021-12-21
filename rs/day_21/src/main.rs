#![deny(clippy::all)]
#![feature(test)]
extern crate test;

use std::{fs::read_to_string, path::Path};

type Input = [u16; 2];

fn main() {
    let data = read_data("./data");

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

fn read_data(data_dir: &str) -> Input {
    let contents = read_to_string(Path::new(data_dir).join("day_21.txt")).unwrap();

    parse_contents(contents.trim())
}

fn parse_contents(contents: &str) -> Input {
    let (l_1, l_2) = contents.split_once('\n').unwrap();

    [
        l_1.rsplit(' ').next().unwrap().parse::<u16>().unwrap(),
        l_2.rsplit(' ').next().unwrap().parse::<u16>().unwrap(),
    ]
}

fn part_1(input: &Input) -> usize {
    let mut die = (1..=100).cycle();

    let mut positions = input.to_owned();
    let mut scores = [0u16, 0u16];
    for rolls in (1..).step_by(2) {
        for player in 0..=1 {
            let new_position = ((positions[player]
                + die.next().unwrap()
                + die.next().unwrap()
                + die.next().unwrap()
                - 1)
                % 10)
                + 1;
            positions[player] = new_position;
            scores[player] += new_position;
            if scores[player] >= 1000 {
                return 3 * (rolls + player) * scores[1 - player] as usize;
            }
        }
    }
    0
}

fn part_2(input: &Input) -> usize {
    recursive_die_p0(input.to_owned(), [0u16; 2], 1)
        .iter()
        .max()
        .unwrap()
        .to_owned()
}

const PROBABILITIES: [(u16, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn recursive_die_p0(positions: [u16; 2], scores: [u16; 2], probability: usize) -> [usize; 2] {
    let mut universes = [0usize; 2];
    for (total_roll, probability) in PROBABILITIES {
        let new_position = ((positions[0] + total_roll - 1) % 10) + 1;
        let new_score = scores[0] + new_position;
        if new_score >= 21 {
            universes[0] += probability;
            continue;
        }
        let new_universes = recursive_die_p1(
            [new_position, positions[1]],
            [new_score, scores[1]],
            probability,
        );
        universes = [
            universes[0] + new_universes[0],
            universes[1] + new_universes[1],
        ];
    }
    [universes[0] * probability, universes[1] * probability]
}

fn recursive_die_p1(positions: [u16; 2], scores: [u16; 2], probability: usize) -> [usize; 2] {
    let mut universes = [0usize; 2];
    for (total_roll, probability) in PROBABILITIES {
        let new_position = ((positions[1] + total_roll - 1) % 10) + 1;
        let new_score = scores[1] + new_position;
        if new_score >= 21 {
            universes[1] += probability;
            continue;
        }
        let new_universes = recursive_die_p0(
            [positions[0], new_position],
            [scores[0], new_score],
            probability,
        );
        universes = [
            universes[0] + new_universes[0],
            universes[1] + new_universes[1],
        ];
    }
    [universes[0] * probability, universes[1] * probability]
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const PART_1: usize = 802452;
    const PART_2: usize = 270005289024391;

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
                input: "Player 1 starting position: 4
Player 2 starting position: 8",
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
                expected: 739785,
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
                expected: 444356092776315,
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
        [4, 8]
    }
}
