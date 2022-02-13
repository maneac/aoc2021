#![deny(clippy::all)]
#![feature(test)]
extern crate test;

use std::{fs::read_to_string, path::Path};

pub type Input = [[isize; 2]; 2];

pub const PART_1: usize = 12090;
pub const PART_2: usize = 5059;

pub fn read_data(data_dir: &str) -> Input {
    let contents = read_to_string(Path::new(data_dir).join("day_17.txt")).unwrap();

    parse_contents(contents.trim())
}

fn parse_contents(contents: &str) -> Input {
    let (lhs, rhs) = contents.split_once(", ").unwrap();

    let (_, x_range) = lhs.split_once('=').unwrap();
    let (_, y_range) = rhs.split_once('=').unwrap();

    let (x_l_str, x_r_str) = x_range.split_once("..").unwrap();
    let (y_l_str, y_r_str) = y_range.split_once("..").unwrap();

    let (x_l, x_r) = (
        x_l_str.parse::<isize>().unwrap(),
        x_r_str.parse::<isize>().unwrap(),
    );

    let (y_l, y_r) = (
        y_l_str.parse::<isize>().unwrap(),
        y_r_str.parse::<isize>().unwrap(),
    );

    [[x_l, x_r], [y_l, y_r]]
}

pub fn part_1(input: &Input) -> usize {
    let best_y = find_max_y(input);

    (best_y * (best_y + 1)) / 2
}

pub fn part_2(input: &Input) -> usize {
    // min x when x=0 on target boundary
    // => x(x+1)/2 = min x
    let min_x = (((1.0 + (8.0 * input[0][0] as f64)).sqrt() - 1.0) / 2.0).ceil() as isize;

    let max_y = find_max_y(input) as isize;

    let valid_xs = (min_x..=input[0][1])
        .filter(|x| can_x_achieve_range(input, x))
        .collect::<Vec<isize>>();

    let valid_ys = (input[1][0]..=max_y)
        .filter(|y| can_y_achieve_range(input, y))
        .collect::<Vec<isize>>();

    valid_xs
        .iter()
        .flat_map(|x| std::iter::repeat(x).zip(&valid_ys))
        .filter(|&(x, y)| {
            let (mut v_x, mut v_y) = (*x, *y);
            let (mut s_x, mut s_y) = (0, 0);

            loop {
                s_x += v_x;
                s_y += v_y;
                if (input[0][0]..=input[0][1]).contains(&s_x)
                    && (input[1][0]..=input[1][1]).contains(&s_y)
                {
                    return true;
                }
                if s_x.gt(&input[0][1]) || s_y.lt(&input[1][0]) {
                    return false;
                }
                v_x -= v_x.signum();
                v_y -= 1;
            }
        })
        .count()
}

fn find_max_y(input: &Input) -> usize {
    for y in (0..-input[1][0]).rev() {
        let mut v_y = -(y + 1);
        let mut y_sum = 0;
        while y_sum.ge(&input[1][0]) {
            y_sum += v_y;
            v_y -= 1;
            if (input[1][0]..=input[1][1]).contains(&y_sum) {
                return y as usize;
            }
        }
    }
    0
}

fn can_x_achieve_range(input: &Input, x: &isize) -> bool {
    let mut v_x = *x;
    let mut s_x = 0;
    loop {
        s_x += v_x;
        if (input[0][0]..=input[0][1]).contains(&s_x) {
            return true;
        }
        if v_x == 0 || s_x.gt(&input[0][1]) {
            return false;
        }
        v_x -= v_x.signum();
    }
}

fn can_y_achieve_range(input: &Input, y: &isize) -> bool {
    let mut v_y = *y;
    let mut s_y = 0;
    loop {
        s_y += v_y;
        if (input[1][0]..=input[1][1]).contains(&s_y) {
            return true;
        }
        if s_y.lt(&input[1][0]) {
            return false;
        }
        v_y -= 1;
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
                input: "target area: x=20..30, y=-10..-5",
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
                expected: 45,
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
                expected: 112,
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
        [[20, 30], [-10, -5]]
    }
}
