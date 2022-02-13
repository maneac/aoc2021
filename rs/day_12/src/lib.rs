#![deny(clippy::all)]
#![feature(test)]
extern crate test;

use std::{cmp::Ordering, fs::read_to_string, path::Path};

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
pub struct Input {
    nodes: Vec<Node>,
}

pub const PART_1: usize = 4912;
pub const PART_2: usize = 150004;

pub fn read_data(data_dir: &str) -> Input {
    let contents = read_to_string(Path::new(data_dir).join("day_12.txt")).unwrap();

    parse_contents(contents.trim())
}

fn parse_contents(contents: &str) -> Input {
    let (mut nodes, edges) =
        contents
            .lines()
            .fold((Vec::new(), Vec::new()), |(mut nodes, mut edges), line| {
                let (lhs, rhs) = line.split_once('-').unwrap();

                for node in [lhs, rhs] {
                    if !nodes.contains(&node.to_string()) {
                        nodes.push(node.to_owned());
                    }
                }
                edges.push((lhs.to_owned(), rhs.to_owned()));

                (nodes, edges)
            });

    nodes.sort_unstable_by(|a, b| {
        if a == "start" || b == "end" {
            return Ordering::Less;
        }
        if a == "end" || b == "start" {
            return Ordering::Greater;
        }
        a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase())
    });

    let mut output = vec![Node::default(); nodes.len()];

    output
        .iter_mut()
        .zip(&nodes)
        .for_each(|(o, n)| o.large = n.to_ascii_lowercase() != *n);

    edges.iter().for_each(|(lhs, rhs)| {
        let lhs_idx = nodes.iter().position(|n| n == lhs).unwrap();
        let rhs_idx = nodes.iter().position(|n| n == rhs).unwrap();
        output[lhs_idx].connections.push(rhs_idx);
        output[rhs_idx].connections.push(lhs_idx);
    });

    output
        .iter_mut()
        .for_each(|n| n.connections.sort_unstable());

    Input { nodes: output }
}

pub fn part_1(input: &Input) -> usize {
    input.paths_small_once(0, 1)
}

pub fn part_2(input: &Input) -> usize {
    input.paths_single_small_twice(0, 1, false)
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
struct Node {
    large: bool,
    connections: Vec<usize>,
}

impl Input {
    fn paths_small_once(&self, idx: usize, path: u16) -> usize {
        self.nodes[idx]
            .connections
            .iter()
            .filter(|&&conn| self.nodes[conn].large || (path & (1 << conn) == 0))
            .fold(0usize, |acc, &conn| {
                if conn == self.nodes.len() - 1 {
                    acc + 1
                } else {
                    acc + self.paths_small_once(conn, path | (1 << conn))
                }
            })
    }

    fn paths_single_small_twice(&self, idx: usize, path: u16, visited_second_small: bool) -> usize {
        self.nodes[idx]
            .connections
            .iter()
            .filter(|&&conn| {
                self.nodes[conn].large || !visited_second_small || (path & (1 << conn) == 0)
            })
            .fold(0usize, |acc, &conn| match conn {
                0 => acc,
                _ if conn == self.nodes.len() - 1 => acc + 1,
                _ => {
                    acc + self.paths_single_small_twice(
                        conn,
                        path | (1 << conn),
                        visited_second_small || (!self.nodes[conn].large && path & (1 << conn) > 0),
                    )
                }
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
            run(&Case {
                input: "start-A
start-b
A-c
A-b
b-d
A-end
b-end",
                expected: example_1_data(),
            })
        }

        #[test]
        fn example_2() {
            run(&Case {
                input: "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc",
                expected: example_2_data(),
            })
        }

        #[test]
        fn example_3() {
            run(&Case {
                input: "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW",
                expected: example_3_data(),
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
                expected: 10,
            })
        }

        #[test]
        fn example_2() {
            run(&Case {
                data: example_2_data(),
                expected: 19,
            })
        }

        #[test]
        fn example_3() {
            run(&Case {
                data: example_3_data(),
                expected: 226,
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
                expected: 36,
            })
        }

        #[test]
        fn example_2() {
            run(&Case {
                data: example_2_data(),
                expected: 103,
            })
        }

        #[test]
        fn example_3() {
            run(&Case {
                data: example_3_data(),
                expected: 3509,
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

    fn example_1_data() -> Input {
        Input {
            nodes: vec![
                // start
                Node {
                    large: false,
                    connections: vec![1, 2],
                },
                // A
                Node {
                    large: true,
                    connections: vec![0, 2, 3, 5],
                },
                // b
                Node {
                    large: false,
                    connections: vec![0, 1, 4, 5],
                },
                // c
                Node {
                    large: false,
                    connections: vec![1],
                },
                // d
                Node {
                    large: false,
                    connections: vec![2],
                },
                // end
                Node {
                    large: false,
                    connections: vec![1, 2],
                },
            ],
        }
    }

    fn example_2_data() -> Input {
        Input {
            nodes: vec![
                // start
                Node {
                    large: false,
                    connections: vec![1, 2, 3],
                },
                // dc
                Node {
                    large: false,
                    connections: vec![0, 2, 3, 4, 6],
                },
                // HN
                Node {
                    large: true,
                    connections: vec![0, 1, 3, 6],
                },
                // kj
                Node {
                    large: false,
                    connections: vec![0, 1, 2, 5],
                },
                // LN
                Node {
                    large: true,
                    connections: vec![1],
                },
                // sa
                Node {
                    large: false,
                    connections: vec![3],
                },
                // end
                Node {
                    large: false,
                    connections: vec![1, 2],
                },
            ],
        }
    }

    fn example_3_data() -> Input {
        Input {
            nodes: vec![
                // start 0
                Node {
                    large: false,
                    connections: vec![1, 4, 5],
                },
                // DX 1
                Node {
                    large: true,
                    connections: vec![0, 2, 3, 4],
                },
                // fs 2
                Node {
                    large: false,
                    connections: vec![1, 3, 4, 9],
                },
                // he 3
                Node {
                    large: false,
                    connections: vec![1, 2, 4, 5, 7, 8],
                },
                // pj 4
                Node {
                    large: false,
                    connections: vec![0, 1, 2, 3, 5, 8],
                },
                // RW 5
                Node {
                    large: true,
                    connections: vec![0, 3, 4, 8],
                },
                // sl 6
                Node {
                    large: false,
                    connections: vec![8],
                },
                // WI 7
                Node {
                    large: true,
                    connections: vec![3],
                },
                // zg 8
                Node {
                    large: false,
                    connections: vec![3, 4, 5, 6, 9],
                },
                // end 9
                Node {
                    large: false,
                    connections: vec![2, 8],
                },
            ],
        }
    }
}
