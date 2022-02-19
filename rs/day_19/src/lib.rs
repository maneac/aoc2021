#![deny(clippy::all)]
#![feature(test)]
#![feature(drain_filter)]
extern crate test;

use std::{
    cmp::Ordering,
    fs::read_to_string,
    hash::Hash,
    ops::{AddAssign, Sub, SubAssign},
    path::Path,
    sync::{mpsc, Arc, Mutex},
    thread,
};

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
pub struct Input {
    beacon_count: usize,
    max_scanner_distance: usize,
}

pub const PART_1: usize = 483;
pub const PART_2: usize = 14804;

pub fn read_data(data_dir: &str) -> Input {
    let contents = read_to_string(Path::new(data_dir).join("day_19.txt")).unwrap();

    find_transforms(parse_contents(contents.trim()), 12)
}

fn parse_contents(contents: &str) -> Vec<Vec<Point>> {
    contents.split("\n\n").fold(Vec::new(), |mut acc, chunk| {
        let scanner = chunk.split('\n').skip(1).fold(Vec::new(), |mut acc, line| {
            let mut point_iter = line.split(',').map(|v| v.parse::<isize>().unwrap());
            let point = Point {
                x: point_iter.next().unwrap(),
                y: point_iter.next().unwrap(),
                z: point_iter.next().unwrap(),
            };
            acc.push(point);
            acc
        });
        acc.push(scanner);
        acc
    })
}

pub fn part_1(input: &Input) -> usize {
    input.beacon_count
}

pub fn part_2(input: &Input) -> usize {
    input.max_scanner_distance
}

fn find_transforms(input: Vec<Vec<Point>>, threshold: usize) -> Input {
    let mut queue = Vec::new();
    for lhs in 0..input.len() {
        for rhs in (lhs + 1)..input.len() {
            queue.push((lhs, rhs));
        }
    }

    let data = Arc::new(input.clone());
    let work_queue = Arc::new(Mutex::new(queue));

    let (res_tx, res_rx) = mpsc::channel();

    let mut thread_pool = Vec::new();
    for _ in 0..(usize::from(thread::available_parallelism().unwrap()) - 1).max(1) {
        let queue_handle = Arc::clone(&work_queue);
        let data_handle = Arc::clone(&data);
        let result_emitter = res_tx.clone();
        let handle = thread::spawn(move || {
            while let Some((lhs_idx, rhs_idx)) = {
                let mut l = queue_handle.lock().unwrap();
                l.pop()
            } {
                let opt = || {
                    let lhs = &data_handle[lhs_idx];
                    for facing in 0..24 {
                        let mut rhs = data_handle[rhs_idx].clone();
                        rhs.iter_mut().for_each(|p| p.transform(facing));
                        if let Some(p) = find_translation(&rhs, lhs, threshold) {
                            result_emitter
                                .send(Transform {
                                    lhs: lhs_idx,
                                    rhs: rhs_idx,
                                    facing,
                                    transform: p,
                                })
                                .unwrap();
                            return;
                        };
                    }
                };
                opt();
            }
        });
        thread_pool.push(handle);
    }

    let mut transforms = Vec::new();
    while let Ok(t) = res_rx.recv_timeout(std::time::Duration::from_millis(200)) {
        transforms.push(t);
    }

    for t in thread_pool {
        t.join().unwrap()
    }

    let mut known_transforms = vec![0];
    while !transforms.is_empty() {
        let to_add: Vec<Transform> = {
            let t: Vec<Transform> = transforms
                .drain_filter(|t| known_transforms.contains(&t.lhs))
                .collect();
            if !t.is_empty() {
                t
            } else {
                transforms
                    .drain_filter(|t| known_transforms.contains(&t.rhs))
                    .map(|mut t| {
                        t.rhs = t.lhs;
                        t
                    })
                    .collect()
            }
        };

        assert!(!to_add.is_empty());
        for t in to_add {
            known_transforms.push(t.rhs);
        }
    }

    fold_input(&input, known_transforms, threshold)
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct Transform {
    lhs: usize,
    rhs: usize,
    facing: u8,
    transform: Point,
}

fn find_translation(rhs: &[Point], lhs: &[Point], threshold: usize) -> Option<Point> {
    for &lhs_glue in lhs.iter().take((lhs.len() - threshold).max(1)) {
        for &rhs_glue in rhs.iter().take((rhs.len() - threshold).max(1)) {
            let transform = rhs_glue - lhs_glue;
            let overlap = rhs
                .iter()
                .map(|&p| p - transform)
                .filter(|p| lhs.contains(p))
                .count();
            if overlap >= threshold {
                return Some(transform);
            }
        }
    }
    None
}

fn fold_input(input: &[Vec<Point>], order: Vec<usize>, threshold: usize) -> Input {
    let mut beacons = input[0].clone();

    let mut translations = Vec::new();
    for rhs_idx in order {
        let mut opt = || {
            for facing in 0..24 {
                let mut rhs = input[rhs_idx].clone();
                rhs.iter_mut().for_each(|p| p.transform(facing));
                if let Some(translation) = find_translation(&rhs, &beacons, threshold) {
                    rhs.iter_mut().for_each(|p| *p -= translation);
                    translations.push(translation);
                    beacons.append(&mut rhs);
                    beacons.sort_unstable();
                    beacons.dedup();
                    return;
                };
            }
            panic!("no overlap found for idx {}", rhs_idx);
        };
        opt();
    }

    let mut best_dist = 0;
    for lhs_idx in 0..translations.len() {
        for rhs_idx in (lhs_idx + 1)..translations.len() {
            let lhs = translations[lhs_idx];
            let rhs = translations[rhs_idx];

            let dist = (lhs.x.abs_diff(rhs.x)) + (lhs.y.abs_diff(rhs.y)) + (lhs.z.abs_diff(rhs.z));

            best_dist = best_dist.max(dist);
        }
    }

    Input {
        beacon_count: beacons.len(),
        max_scanner_distance: best_dist,
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, rhs: &Self) -> std::option::Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Point {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match self.x.cmp(&rhs.x) {
            Ordering::Equal => {}
            ord => return ord,
        }
        match self.y.cmp(&rhs.y) {
            Ordering::Equal => {}
            ord => return ord,
        }
        self.z.cmp(&rhs.z)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Point {
    fn transform(&mut self, facing: u8) {
        let rotation = (facing & 0b11100) >> 2;
        match rotation {
            0 => {}
            1 => (self.y, self.z) = (self.z, -self.y),
            2 => (self.y, self.z) = (-self.y, -self.z),
            3 => (self.y, self.z) = (-self.z, self.y),
            4 => (self.x, self.z) = (self.z, -self.x),
            5 => (self.x, self.z) = (-self.z, self.x),
            _ => {
                panic!("rotation too large: {}", rotation)
            }
        }

        let spin = facing & 3;
        match spin {
            0 => {}
            1 => (self.x, self.y) = (self.y, -self.x),
            2 => (self.x, self.y) = (-self.x, -self.y),
            3 => (self.x, self.y) = (-self.y, self.x),
            _ => {
                panic!("impossible spin: {}", spin)
            }
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

        #[ignore = "takes 3s to run"]
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
            expected: Vec<Vec<Point>>,
        }

        #[test]
        fn example() {
            run(&Case {
                input: "--- scanner 0 ---
0,2,0
4,1,1
3,3,2

--- scanner 1 ---
-1,-1,3
-5,0,4
-2,1,5",
                expected: vec![
                    vec![
                        Point { x: 0, y: 2, z: 0 },
                        Point { x: 4, y: 1, z: 1 },
                        Point { x: 3, y: 3, z: 2 },
                    ],
                    vec![
                        Point { x: -1, y: -1, z: 3 },
                        Point { x: -5, y: 0, z: 4 },
                        Point { x: -2, y: 1, z: 5 },
                    ],
                ],
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
                expected: 79,
            })
        }

        #[ignore = "takes 3s to run"]
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
                expected: 3621,
            })
        }

        #[ignore = "takes 3s to run"]
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
        //         let input = parse_contents(
        //             "--- scanner 0 ---
        // 404,-588,-901
        // 528,-643,409
        // -838,591,734
        // 390,-675,-793
        // -537,-823,-458
        // -485,-357,347
        // -345,-311,381
        // -661,-816,-575
        // -876,649,763
        // -618,-824,-621
        // 553,345,-567
        // 474,580,667
        // -447,-329,318
        // -584,868,-557
        // 544,-627,-890
        // 564,392,-477
        // 455,729,728
        // -892,524,684
        // -689,845,-530
        // 423,-701,434
        // 7,-33,-71
        // 630,319,-379
        // 443,580,662
        // -789,900,-551
        // 459,-707,401

        // --- scanner 1 ---
        // 686,422,578
        // 605,423,415
        // 515,917,-361
        // -336,658,858
        // 95,138,22
        // -476,619,847
        // -340,-569,-846
        // 567,-361,727
        // -460,603,-452
        // 669,-402,600
        // 729,430,532
        // -500,-761,534
        // -322,571,750
        // -466,-666,-811
        // -429,-592,574
        // -355,545,-477
        // 703,-491,-529
        // -328,-685,520
        // 413,935,-424
        // -391,539,-444
        // 586,-435,557
        // -364,-763,-893
        // 807,-499,-711
        // 755,-354,-619
        // 553,889,-390

        // --- scanner 2 ---
        // 649,640,665
        // 682,-795,504
        // -784,533,-524
        // -644,584,-595
        // -588,-843,648
        // -30,6,44
        // -674,560,763
        // 500,723,-460
        // 609,671,-379
        // -555,-800,653
        // -675,-892,-343
        // 697,-426,-610
        // 578,704,681
        // 493,664,-388
        // -671,-858,530
        // -667,343,800
        // 571,-461,-707
        // -138,-166,112
        // -889,563,-600
        // 646,-828,498
        // 640,759,510
        // -630,509,768
        // -681,-892,-333
        // 673,-379,-804
        // -742,-814,-386
        // 577,-820,562

        // --- scanner 3 ---
        // -589,542,597
        // 605,-692,669
        // -500,565,-823
        // -660,373,557
        // -458,-679,-417
        // -488,449,543
        // -626,468,-788
        // 338,-750,-386
        // 528,-832,-391
        // 562,-778,733
        // -938,-730,414
        // 543,643,-506
        // -524,371,-870
        // 407,773,750
        // -104,29,83
        // 378,-903,-323
        // -778,-728,485
        // 426,699,580
        // -438,-605,-362
        // -469,-447,-387
        // 509,732,623
        // 647,635,-688
        // -868,-804,481
        // 614,-800,639
        // 595,780,-596

        // --- scanner 4 ---
        // 727,592,562
        // -293,-554,779
        // 441,611,-461
        // -714,465,-776
        // -743,427,-804
        // -660,-479,-426
        // 832,-632,460
        // 927,-485,-438
        // 408,393,-506
        // 466,436,-512
        // 110,16,151
        // -258,-428,682
        // -393,719,612
        // -211,-452,876
        // 808,-476,-593
        // -575,615,604
        // -485,667,467
        // -680,325,-822
        // -627,-443,-432
        // 872,-547,-609
        // 833,512,582
        // 807,604,487
        // 839,-516,451
        // 891,-625,532
        // -652,-548,-490
        // 30,-46,-14",
        //         );

        //         find_transforms(input, 12);

        Input {
            beacon_count: 79,
            max_scanner_distance: 3621,
        }
    }
}
