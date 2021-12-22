#![feature(destructuring_assignment)]
#![feature(test)]
extern crate test;

use std::{
    cmp::Ordering,
    ops::{AddAssign, Sub, SubAssign},
};

#[derive(Debug, PartialEq, PartialOrd, Default)]
struct Input {
    beacons: Vec<Point>,
    scanner_locations: Vec<Point>,
}

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
struct Point {
    p: [isize; 4],
}

impl PartialOrd for Point {
    fn partial_cmp(&self, rhs: &Self) -> std::option::Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Point {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match self.p[0].cmp(&rhs.p[0]) {
            Ordering::Equal => {}
            ord => return ord,
        }
        match self.p[1].cmp(&rhs.p[1]) {
            Ordering::Equal => {}
            ord => return ord,
        }
        self.p[2].cmp(&rhs.p[2])
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            p: [
                self.p[0] - rhs.p[0],
                self.p[1] - rhs.p[1],
                self.p[2] - rhs.p[2],
                self.p[3] - rhs.p[3],
            ],
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..4 {
            self.p[i] -= rhs.p[i];
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..4 {
            self.p[i] += rhs.p[i];
        }
    }
}

fn foo(facing: u8) -> [[isize; 4]; 4] {
    match facing {
        0 => [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
        1 => [[0, 1, 0, 0], [-1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        2 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        3 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        4 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        5 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        6 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        7 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        8 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        9 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        10 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        11 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        12 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        13 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        14 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        15 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        16 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        17 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        18 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        19 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        20 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        21 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        22 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        23 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        _ => {
            panic!("invalid facing: {}", facing)
        }
    }
}

impl Point {
    fn transform(&mut self, facing: u8) {
        match facing {
            0 => {}
            1 => (self.x, self.y) = (self.y, -self.x),
            2 => (self.x, self.y) = (-self.x, -self.y),
            3 => (self.x, self.y) = (-self.y, self.x),
            4 => (self.y, self.z) = (self.z, -self.y),
            5 => (self.x, self.y, self.z) = (self.z, -self.x, -self.y),
            6 => (self.x, self.y, self.z) = (-self.x, -self.z, -self.y),
            7 => (self.x, self.y, self.z) = (-self.z, self.x, -self.y),
            8 => (self.y, self.z) = (-self.y, -self.z),
            9 => (self.x, self.y, self.z) = (-self.y, -self.x, -self.z),
            10 => (self.x, self.y, self.z) = (-self.x, self.y, -self.z),
            11 => (self.x, self.y, self.z) = (self.y, self.x, -self.z),
            12 => (self.y, self.z) = (-self.z, self.y),
            13 => (self.x, self.y, self.z) = (-self.z, -self.x, self.y),
            14 => (self.x, self.y, self.z) = (-self.x, self.z, self.y),
            15 => (self.x, self.y, self.z) = (self.z, self.x, self.y),
            16 => (self.x, self.z) = (self.z, -self.x),
            17 => (self.x, self.y, self.z) = (self.y, -self.z, -self.x),
            18 => (self.x, self.y, self.z) = (-self.z, -self.y, -self.x),
            19 => (self.x, self.y, self.z) = (-self.y, self.z, -self.x),
            20 => (self.x, self.z) = (-self.z, self.x),
            21 => (self.x, self.y, self.z) = (self.y, self.z, self.x),
            22 => (self.x, self.y, self.z) = (self.z, -self.y, self.x),
            23 => (self.x, self.y, self.z) = (-self.y, -self.z, self.x),
            _ => {
                panic!("invalid facing: {}", facing)
            }
        }
    }

    fn inverse_transform(&mut self, facing: u8) {
        let inverse_facing = match facing {
            0 => 0,
            1 => 3,
            2 => 2,
            3 => 1,
            4 => 12,
            5 => 15,
            6 => 14,
            7 => 13,
            8 => 8,
            9 => 11,
            10 => 10,
            11 => 9,
            12 => 4,
            13 => 7,
            14 => 6,
            15 => 5,
            16 => 20,
            17 => 23,
            18 => 22,
            19 => 21,
            20 => 16,
            21 => 19,
            22 => 18,
            23 => 17,
            _ => {
                panic!("invalid facing: {}", facing)
            }
        };
        self.transform(inverse_facing);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod transforms {
        use super::*;
        use test::Bencher;

        #[bench]
        fn naive(b: &mut Bencher) {
            let mut data = example_data();

            b.iter(|| {
                for i in 0..24 {
                    data.beacons.iter_mut().map(|b| b.transform(i)).count();
                }
            })
        }

        #[bench]
        fn vectorised(b: &mut Bencher) {
            let mut data = example_data();

            b.iter(|| {
                for i in 0..24 {
                    let transform = foo(i);
                    data.beacons
                        .iter_mut()
                        .map(|b| {
                            let v = [b.x, b.y, b.z, b.a];
                            *b = Point {
                                x: v.iter().zip(transform[0]).map(|(v, t)| v * t).sum(),
                                y: v.iter().zip(transform[1]).map(|(v, t)| v * t).sum(),
                                z: v.iter().zip(transform[2]).map(|(v, t)| v * t).sum(),
                                a: v.iter().zip(transform[3]).map(|(v, t)| v * t).sum(),
                            };
                        })
                        .count();
                }
            })
        }
    }

    fn example_data() -> Input {
        Input {
            beacons: vec![
                Point {
                    x: -892,
                    y: 524,
                    z: 684,
                    a: 0,
                },
                Point {
                    x: -876,
                    y: 649,
                    z: 763,
                    a: 0,
                },
                Point {
                    x: -838,
                    y: 591,
                    z: 734,
                    a: 0,
                },
                Point {
                    x: -789,
                    y: 900,
                    z: -551,
                    a: 0,
                },
                Point {
                    x: -739,
                    y: -1745,
                    z: 668,
                    a: 0,
                },
                Point {
                    x: -706,
                    y: -3180,
                    z: -659,
                    a: 0,
                },
                Point {
                    x: -697,
                    y: -3072,
                    z: -689,
                    a: 0,
                },
                Point {
                    x: -689,
                    y: 845,
                    z: -530,
                    a: 0,
                },
                Point {
                    x: -687,
                    y: -1600,
                    z: 576,
                    a: 0,
                },
                Point {
                    x: -661,
                    y: -816,
                    z: -575,
                    a: 0,
                },
                Point {
                    x: -654,
                    y: -3158,
                    z: -753,
                    a: 0,
                },
                Point {
                    x: -635,
                    y: -1737,
                    z: 486,
                    a: 0,
                },
                Point {
                    x: -631,
                    y: -672,
                    z: 1502,
                    a: 0,
                },
                Point {
                    x: -624,
                    y: -1620,
                    z: 1868,
                    a: 0,
                },
                Point {
                    x: -620,
                    y: -3212,
                    z: 371,
                    a: 0,
                },
                Point {
                    x: -618,
                    y: -824,
                    z: -621,
                    a: 0,
                },
                Point {
                    x: -612,
                    y: -1695,
                    z: 1788,
                    a: 0,
                },
                Point {
                    x: -601,
                    y: -1648,
                    z: -643,
                    a: 0,
                },
                Point {
                    x: -584,
                    y: 868,
                    z: -557,
                    a: 0,
                },
                Point {
                    x: -537,
                    y: -823,
                    z: -458,
                    a: 0,
                },
                Point {
                    x: -532,
                    y: -1715,
                    z: 1894,
                    a: 0,
                },
                Point {
                    x: -518,
                    y: -1681,
                    z: -600,
                    a: 0,
                },
                Point {
                    x: -499,
                    y: -1607,
                    z: -770,
                    a: 0,
                },
                Point {
                    x: -485,
                    y: -357,
                    z: 347,
                    a: 0,
                },
                Point {
                    x: -470,
                    y: -3283,
                    z: 303,
                    a: 0,
                },
                Point {
                    x: -456,
                    y: -621,
                    z: 1527,
                    a: 0,
                },
                Point {
                    x: -447,
                    y: -329,
                    z: 318,
                    a: 0,
                },
                Point {
                    x: -430,
                    y: -3130,
                    z: 366,
                    a: 0,
                },
                Point {
                    x: -413,
                    y: -627,
                    z: 1469,
                    a: 0,
                },
                Point {
                    x: -345,
                    y: -311,
                    z: 381,
                    a: 0,
                },
                Point {
                    x: -36,
                    y: -1284,
                    z: 1171,
                    a: 0,
                },
                Point {
                    x: -27,
                    y: -1108,
                    z: -65,
                    a: 0,
                },
                Point {
                    x: 7,
                    y: -33,
                    z: -71,
                    a: 0,
                },
                Point {
                    x: 12,
                    y: -2351,
                    z: -103,
                    a: 0,
                },
                Point {
                    x: 26,
                    y: -1119,
                    z: 1091,
                    a: 0,
                },
                Point {
                    x: 346,
                    y: -2985,
                    z: 342,
                    a: 0,
                },
                Point {
                    x: 366,
                    y: -3059,
                    z: 397,
                    a: 0,
                },
                Point {
                    x: 377,
                    y: -2827,
                    z: 367,
                    a: 0,
                },
                Point {
                    x: 390,
                    y: -675,
                    z: -793,
                    a: 0,
                },
                Point {
                    x: 396,
                    y: -1931,
                    z: -563,
                    a: 0,
                },
                Point {
                    x: 404,
                    y: -588,
                    z: -901,
                    a: 0,
                },
                Point {
                    x: 408,
                    y: -1815,
                    z: 803,
                    a: 0,
                },
                Point {
                    x: 423,
                    y: -701,
                    z: 434,
                    a: 0,
                },
                Point {
                    x: 432,
                    y: -2009,
                    z: 850,
                    a: 0,
                },
                Point {
                    x: 443,
                    y: 580,
                    z: 662,
                    a: 0,
                },
                Point {
                    x: 455,
                    y: 729,
                    z: 728,
                    a: 0,
                },
                Point {
                    x: 456,
                    y: -540,
                    z: 1869,
                    a: 0,
                },
                Point {
                    x: 459,
                    y: -707,
                    z: 401,
                    a: 0,
                },
                Point {
                    x: 465,
                    y: -695,
                    z: 1988,
                    a: 0,
                },
                Point {
                    x: 474,
                    y: 580,
                    z: 667,
                    a: 0,
                },
                Point {
                    x: 496,
                    y: -1584,
                    z: 1900,
                    a: 0,
                },
                Point {
                    x: 497,
                    y: -1838,
                    z: -617,
                    a: 0,
                },
                Point {
                    x: 527,
                    y: -524,
                    z: 1933,
                    a: 0,
                },
                Point {
                    x: 528,
                    y: -643,
                    z: 409,
                    a: 0,
                },
                Point {
                    x: 534,
                    y: -1912,
                    z: 768,
                    a: 0,
                },
                Point {
                    x: 544,
                    y: -627,
                    z: -890,
                    a: 0,
                },
                Point {
                    x: 553,
                    y: 345,
                    z: -567,
                    a: 0,
                },
                Point {
                    x: 564,
                    y: 392,
                    z: -477,
                    a: 0,
                },
                Point {
                    x: 568,
                    y: -2007,
                    z: -577,
                    a: 0,
                },
                Point {
                    x: 605,
                    y: -1665,
                    z: 1952,
                    a: 0,
                },
                Point {
                    x: 612,
                    y: -1593,
                    z: 1893,
                    a: 0,
                },
                Point {
                    x: 630,
                    y: 319,
                    z: -379,
                    a: 0,
                },
                Point {
                    x: 686,
                    y: -3108,
                    z: -505,
                    a: 0,
                },
                Point {
                    x: 776,
                    y: -3184,
                    z: -501,
                    a: 0,
                },
                Point {
                    x: 846,
                    y: -3110,
                    z: -434,
                    a: 0,
                },
                Point {
                    x: 1135,
                    y: -1161,
                    z: 1235,
                    a: 0,
                },
                Point {
                    x: 1243,
                    y: -1093,
                    z: 1063,
                    a: 0,
                },
                Point {
                    x: 1660,
                    y: -552,
                    z: 429,
                    a: 0,
                },
                Point {
                    x: 1693,
                    y: -557,
                    z: 386,
                    a: 0,
                },
                Point {
                    x: 1735,
                    y: -437,
                    z: 1738,
                    a: 0,
                },
                Point {
                    x: 1749,
                    y: -1800,
                    z: 1813,
                    a: 0,
                },
                Point {
                    x: 1772,
                    y: -405,
                    z: 1572,
                    a: 0,
                },
                Point {
                    x: 1776,
                    y: -675,
                    z: 371,
                    a: 0,
                },
                Point {
                    x: 1779,
                    y: -442,
                    z: 1789,
                    a: 0,
                },
                Point {
                    x: 1780,
                    y: -1548,
                    z: 337,
                    a: 0,
                },
                Point {
                    x: 1786,
                    y: -1538,
                    z: 337,
                    a: 0,
                },
                Point {
                    x: 1847,
                    y: -1591,
                    z: 415,
                    a: 0,
                },
                Point {
                    x: 1889,
                    y: -1729,
                    z: 1762,
                    a: 0,
                },
                Point {
                    x: 1994,
                    y: -1805,
                    z: 1792,
                    a: 0,
                },
            ],
            scanner_locations: vec![
                Point {
                    x: 0,
                    y: 0,
                    z: 0,
                    a: 0,
                },
                Point {
                    x: -68,
                    y: 1246,
                    z: 43,
                    a: 0,
                },
                Point {
                    x: 92,
                    y: 2380,
                    z: 20,
                    a: 0,
                },
                Point {
                    x: 20,
                    y: 1133,
                    z: -1061,
                    a: 0,
                },
                Point {
                    x: -1105,
                    y: 1205,
                    z: -1229,
                    a: 0,
                },
            ],
        }
    }
}
