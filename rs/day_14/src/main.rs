#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]
#![feature(test)]
extern crate test;

use std::{collections::HashMap, fs::read_to_string, path::Path};

fn main() {
    let data = read_data("./data");

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

fn read_data(data_dir: &str) -> Input {
    let contents = read_to_string(Path::new(data_dir).join("day_14.txt")).unwrap();

    let (input, rules) = contents.trim().split_once("\n\n").unwrap();

    let initial: HashMap<u16, u64> =
        input
            .as_bytes()
            .windows(2)
            .fold(HashMap::new(), |mut acc, pair| {
                let k = (pair[0] as u16) << 8 | pair[1] as u16;
                acc.get_mut(&k).map(|v| *v += 1).unwrap_or_else(|| {
                    acc.insert(k, 1);
                });
                acc
            });

    let insertions = rules
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .fold(HashMap::new(), |mut acc, (pair, elem)| {
            let mut key_iter = pair.bytes();
            acc.insert(
                (key_iter.next().unwrap() as u16) << 8 | key_iter.next().unwrap() as u16,
                elem.as_bytes()[0] as u16,
            );
            acc
        });

    Input {
        first: input.bytes().next().unwrap() - b'A',
        last: input.bytes().last().unwrap() - b'A',
        initial,
        insertions,
    }
}

fn part_1(input: &Input) -> u64 {
    input.polymerize(10)
}

fn part_2(input: &Input) -> u64 {
    input.polymerize(40)
}

#[derive(Debug, PartialEq, Default)]
struct Input {
    first: u8,
    last: u8,
    initial: HashMap<u16, u64>,
    insertions: HashMap<u16, u16>,
}

impl Input {
    fn polymerize(&self, num_iterations: usize) -> u64 {
        let mut polymer: HashMap<u16, u64> = self.initial.clone();
        let mut new_polymer: HashMap<u16, u64> = HashMap::with_capacity(polymer.len());

        (0..num_iterations).for_each(|iter| {
            let (old, new) = if iter & 1 > 0 {
                (&mut new_polymer, &mut polymer)
            } else {
                (&mut polymer, &mut new_polymer)
            };

            old.iter_mut().for_each(|(k, old_count)| {
                let count = *old_count;
                *old_count = 0;
                if let Some(&chr) = self.insertions.get(k) {
                    let lhs = chr | (k & 0xFF00);
                    new.get_mut(&lhs).map(|v| *v += count).unwrap_or_else(|| {
                        new.insert(lhs, count);
                    });

                    let rhs = (chr << 8) | (k & 0xFF);
                    new.get_mut(&rhs).map(|v| *v += count).unwrap_or_else(|| {
                        new.insert(rhs, count);
                    });
                    return;
                }
                new.get_mut(k).map(|v| *v += count).unwrap_or_else(|| {
                    new.insert(*k, count);
                });
            });
        });

        let counts = &polymer.iter().fold([0u64; 26], |mut acc, (k, v)| {
            acc[((k >> 8) as u8 - b'A') as usize] += v;
            acc[(*k as u8 - b'A') as usize] += v;
            acc
        });

        let (smallest, largest) = counts.iter().enumerate().filter(|(_, &c)| c > 0).fold(
            (u64::MAX, u64::MIN),
            |(min, max), (idx, &v)| {
                let count =
                    (v + (idx as u8 == self.first) as u64 + (idx as u8 == self.last) as u64) / 2;
                (min.min(count), max.max(count))
            },
        );

        largest - smallest
    }
}

#[cfg(test)]
mod day_14 {
    use super::*;
    use std::fs::write;
    use test::Bencher;

    const PART_1: u64 = 2584;
    const PART_2: u64 = 3816397135460;

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
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

        write(Path::new("/tmp").join("day_14.txt"), input).unwrap();

        assert_eq!(example_data(), read_data("/tmp"));
    }

    #[test]
    fn test_part_1_example() {
        let data = example_data();

        assert_eq!(1588, part_1(&data));
    }

    #[test]
    fn test_part_2_example() {
        let data = example_data();

        assert_eq!(2188189693529, part_2(&data));
    }

    fn example_data() -> Input {
        let mut initial = HashMap::with_capacity(3);

        initial.insert((b'N' as u16) << 8 | b'N' as u16, 1);
        initial.insert((b'N' as u16) << 8 | b'C' as u16, 1);
        initial.insert((b'C' as u16) << 8 | b'B' as u16, 1);

        let mut insertions = HashMap::with_capacity(16);

        insertions.insert((b'C' as u16) << 8 | b'H' as u16, b'B' as u16);
        insertions.insert((b'H' as u16) << 8 | b'H' as u16, b'N' as u16);
        insertions.insert((b'C' as u16) << 8 | b'B' as u16, b'H' as u16);
        insertions.insert((b'N' as u16) << 8 | b'H' as u16, b'C' as u16);
        insertions.insert((b'H' as u16) << 8 | b'B' as u16, b'C' as u16);
        insertions.insert((b'H' as u16) << 8 | b'C' as u16, b'B' as u16);
        insertions.insert((b'H' as u16) << 8 | b'N' as u16, b'C' as u16);
        insertions.insert((b'N' as u16) << 8 | b'N' as u16, b'C' as u16);
        insertions.insert((b'B' as u16) << 8 | b'H' as u16, b'H' as u16);
        insertions.insert((b'N' as u16) << 8 | b'C' as u16, b'B' as u16);
        insertions.insert((b'N' as u16) << 8 | b'B' as u16, b'B' as u16);
        insertions.insert((b'B' as u16) << 8 | b'N' as u16, b'B' as u16);
        insertions.insert((b'B' as u16) << 8 | b'B' as u16, b'N' as u16);
        insertions.insert((b'B' as u16) << 8 | b'C' as u16, b'B' as u16);
        insertions.insert((b'C' as u16) << 8 | b'C' as u16, b'N' as u16);
        insertions.insert((b'C' as u16) << 8 | b'N' as u16, b'C' as u16);

        Input {
            first: b'N' - b'A',
            last: b'B' - b'A',
            initial,
            insertions,
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