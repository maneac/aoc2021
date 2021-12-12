#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]
#![feature(test)]
extern crate test;

use std::{cmp::Ordering, fs::read_to_string, path::Path};

fn main() {
    let data = read_data("./data");

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

fn read_data(data_dir: &str) -> Input {
    let (mut nodes, edges) = read_to_string(Path::new(data_dir).join("day_12.txt"))
        .unwrap()
        .trim()
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

    dbg!(&nodes);

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

fn part_1(input: &Input) -> usize {
    input.paths_small_once(0, 1)
}

fn part_2(input: &Input) -> usize {
    input.paths_single_small_twice(0, 1, false)
}

#[derive(Debug, PartialEq, PartialOrd, Default)]
struct Input {
    nodes: Vec<Node>,
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
mod day_12 {
    use super::*;
    use std::fs::{create_dir_all, write};
    use test::Bencher;

    const PART_1: usize = 4912;
    const PART_2: usize = 150004;

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
    fn test_read_data_example_1() {
        let data = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

        let path = Path::new("/tmp/day_12_1");
        if !path.exists() {
            create_dir_all(path).unwrap();
        }
        write(path.join("day_12.txt"), data).unwrap();

        assert_eq!(example_1_data(), read_data(path.to_str().unwrap()));
    }

    #[test]
    fn test_read_data_example_2() {
        let data = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

        let path = Path::new("/tmp/day_12_2");
        if !path.exists() {
            create_dir_all(path).unwrap();
        }
        write(path.join("day_12.txt"), data).unwrap();

        assert_eq!(example_2_data(), read_data(path.to_str().unwrap()));
    }

    #[test]
    fn test_read_data_example_3() {
        let data = "fs-end
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
start-RW";

        let path = Path::new("/tmp/day_12_3");
        if !path.exists() {
            create_dir_all(path).unwrap();
        }
        write(path.join("day_12.txt"), data).unwrap();

        assert_eq!(example_3_data(), read_data(path.to_str().unwrap()));
    }

    #[test]
    fn test_part_1_example_1() {
        let data = example_1_data();

        assert_eq!(10, part_1(&data));
    }

    #[test]
    fn test_part_1_example_2() {
        let data = example_2_data();

        assert_eq!(19, part_1(&data));
    }

    #[test]
    fn test_part_1_example_3() {
        let data = example_3_data();

        assert_eq!(226, part_1(&data));
    }

    #[test]
    fn test_part_2_example_1() {
        let data = example_1_data();

        assert_eq!(36, part_2(&data));
    }

    #[test]
    fn test_part_2_example_2() {
        let data = example_2_data();

        assert_eq!(103, part_2(&data));
    }

    #[test]
    fn test_part_2_example_3() {
        let data = example_3_data();

        assert_eq!(3509, part_2(&data));
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
