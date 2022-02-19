#![deny(clippy::all)]
#![feature(test)]
extern crate test;

use rayon::prelude::*;
use std::{collections::HashSet, fs::read_to_string, path::Path};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    instructions: Vec<Instruction>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    min_z: isize,
    max_z: isize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Instruction {
    on: bool,
    bounds: [[isize; 2]; 3],
}

pub const PART_1: usize = 589411;
pub const PART_2: usize = 1130514303649907;

pub fn read_data(data_dir: &str) -> Input {
    let contents = read_to_string(Path::new(data_dir).join("day_22.txt")).unwrap();

    parse_contents(contents.trim())
}

fn parse_contents(contents: &str) -> Input {
    let mut instructions = Vec::with_capacity(contents.lines().count());

    let mut min_x = isize::MAX;
    let mut min_y = isize::MAX;
    let mut min_z = isize::MAX;
    let mut max_x = isize::MIN;
    let mut max_y = isize::MIN;
    let mut max_z = isize::MIN;
    for line in contents.lines() {
        let mut bounds = [[0isize; 2]; 3];
        line.split_terminator('=')
            .skip(1)
            .enumerate()
            .for_each(|(idx, l)| {
                let (min, max) = l
                    .trim_end_matches(&[',', 'x', 'y', 'z'])
                    .split_once("..")
                    .unwrap();
                bounds[idx][0] = min.parse().unwrap();
                bounds[idx][1] = max.parse().unwrap();
                match idx {
                    0 => {
                        min_x = min_x.min(bounds[idx][0]);
                        max_x = max_x.max(bounds[idx][1]);
                    }
                    1 => {
                        min_y = min_y.min(bounds[idx][0]);
                        max_y = max_y.max(bounds[idx][1]);
                    }
                    2 => {
                        min_z = min_z.min(bounds[idx][0]);
                        max_z = max_z.max(bounds[idx][1]);
                    }
                    _ => {}
                }
            });
        match line.chars().nth(1).unwrap() {
            'n' => instructions.push(Instruction { on: true, bounds }),
            'f' => instructions.push(Instruction { on: false, bounds }),
            _ => {
                panic!("invalid instruction: {}", line)
            }
        };
    }

    Input {
        instructions,
        min_x,
        max_x,
        min_y,
        max_y,
        min_z,
        max_z,
    }
}

pub fn part_1(input: &Input) -> usize {
    let mut cube = [[[false; 101]; 101]; 101];
    for instruction in &input.instructions {
        if instruction.bounds[0][0] < -50
            || instruction.bounds[1][0] < -50
            || instruction.bounds[2][0] < -50
            || instruction.bounds[0][1] > 50
            || instruction.bounds[1][1] > 50
            || instruction.bounds[2][1] > 50
        {
            continue;
        }

        for x in instruction.bounds[0][0]..=instruction.bounds[0][1] {
            for y in instruction.bounds[1][0]..=instruction.bounds[1][1] {
                for z in instruction.bounds[2][0]..=instruction.bounds[2][1] {
                    cube[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] = instruction.on;
                }
            }
        }
    }
    cube.iter().flatten().flatten().filter(|v| **v).count()
}

pub fn part_2(input: &Input) -> usize {
    let mut x_bound_set = HashSet::new();
    x_bound_set.insert((input.min_x * 2) - 1);
    x_bound_set.insert((input.max_x * 2) + 1);
    let mut y_bound_set = HashSet::new();
    y_bound_set.insert((input.min_y * 2) - 1);
    y_bound_set.insert((input.max_y * 2) + 1);
    let mut z_bound_set = HashSet::new();
    z_bound_set.insert((input.min_z * 2) - 1);
    z_bound_set.insert((input.max_z * 2) + 1);

    for instruction in &input.instructions {
        x_bound_set.insert((instruction.bounds[0][0] * 2) - 1);
        x_bound_set.insert((instruction.bounds[0][1] * 2) + 1);
        y_bound_set.insert((instruction.bounds[1][0] * 2) - 1);
        y_bound_set.insert((instruction.bounds[1][1] * 2) + 1);
        z_bound_set.insert((instruction.bounds[2][0] * 2) - 1);
        z_bound_set.insert((instruction.bounds[2][1] * 2) + 1);
    }

    let mut x_boundaries = Vec::<isize>::from_iter(x_bound_set);
    x_boundaries.sort_unstable();

    let mut y_boundaries = Vec::<isize>::from_iter(y_bound_set);
    y_boundaries.sort_unstable();

    let mut z_boundaries = Vec::<isize>::from_iter(z_bound_set);
    z_boundaries.sort_unstable();

    x_boundaries
        .windows(2)
        .par_bridge()
        .map(|x_range| {
            let mut total = 0;
            for y_range in y_boundaries.windows(2) {
                for z_range in z_boundaries.windows(2) {
                    let mut on = false;
                    for instruction in &input.instructions {
                        let x_min = 2 * instruction.bounds[0][0];
                        let x_max = 2 * instruction.bounds[0][1];
                        let y_min = 2 * instruction.bounds[1][0];
                        let y_max = 2 * instruction.bounds[1][1];
                        let z_min = 2 * instruction.bounds[2][0];
                        let z_max = 2 * instruction.bounds[2][1];

                        if (x_min.clamp(x_range[0], isize::MAX)
                            ..=x_max.clamp(isize::MIN, x_range[1]))
                            .is_empty()
                            || (y_min.clamp(y_range[0], isize::MAX)
                                ..=y_max.clamp(isize::MIN, y_range[1]))
                                .is_empty()
                            || (z_min.clamp(z_range[0], isize::MAX)
                                ..=z_max.clamp(isize::MIN, z_range[1]))
                                .is_empty()
                        {
                            continue;
                        }

                        on = instruction.on;
                    }
                    if on {
                        total += (x_range[0].abs_diff(x_range[1]) / 2)
                            * (y_range[0].abs_diff(y_range[1]) / 2)
                            * (z_range[0].abs_diff(z_range[1]) / 2);
                    }
                }
            }
            total
        })
        .sum()
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
        fn small_example() {
            run(&Case {
                input: "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10",
                expected: small_example_data(),
            })
        }

        #[test]
        fn large_example() {
            run(&Case {
                input: "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682",
                expected: large_example_data(),
            })
        }

        #[test]
        fn extra_large_example() {
            run(&Case {
                input: "on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507",
                expected: extra_large_example_data(),
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
        fn small_example() {
            run(&Case {
                data: small_example_data(),
                expected: 39,
            })
        }

        #[test]
        fn large_example() {
            run(&Case {
                data: large_example_data(),
                expected: 590784,
            })
        }

        #[test]
        fn extra_large_example() {
            run(&Case {
                data: extra_large_example_data(),
                expected: 474140,
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
        fn small_example() {
            run(&Case {
                data: small_example_data(),
                expected: 39,
            })
        }

        #[test]
        fn large_example() {
            run(&Case {
                data: large_example_data(),
                expected: 590784
                    + (((-54112isize).abs_diff(-39298) + 1)
                        * ((-85059isize).abs_diff(-49293) + 1)
                        * ((-27449isize).abs_diff(7877) + 1))
                    + ((967isize.abs_diff(23432) + 1)
                        * (45373isize.abs_diff(81175) + 1)
                        * (27513isize.abs_diff(53682) + 1)),
            })
        }

        #[test]
        fn extra_large_example() {
            run(&Case {
                data: extra_large_example_data(),
                expected: 2758514936282235,
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

    fn small_example_data() -> Input {
        Input {
            instructions: vec![
                Instruction {
                    on: true,
                    bounds: [[10, 12], [10, 12], [10, 12]],
                },
                Instruction {
                    on: true,
                    bounds: [[11, 13], [11, 13], [11, 13]],
                },
                Instruction {
                    on: false,
                    bounds: [[9, 11], [9, 11], [9, 11]],
                },
                Instruction {
                    on: true,
                    bounds: [[10, 10], [10, 10], [10, 10]],
                },
            ],
            min_x: 9,
            max_x: 13,
            min_y: 9,
            max_y: 13,
            min_z: 9,
            max_z: 13,
        }
    }

    fn large_example_data() -> Input {
        Input {
            instructions: vec![
                Instruction {
                    on: true,
                    bounds: [[-20, 26], [-36, 17], [-47, 7]],
                },
                Instruction {
                    on: true,
                    bounds: [[-20, 33], [-21, 23], [-26, 28]],
                },
                Instruction {
                    on: true,
                    bounds: [[-22, 28], [-29, 23], [-38, 16]],
                },
                Instruction {
                    on: true,
                    bounds: [[-46, 7], [-6, 46], [-50, -1]],
                },
                Instruction {
                    on: true,
                    bounds: [[-49, 1], [-3, 46], [-24, 28]],
                },
                Instruction {
                    on: true,
                    bounds: [[2, 47], [-22, 22], [-23, 27]],
                },
                Instruction {
                    on: true,
                    bounds: [[-27, 23], [-28, 26], [-21, 29]],
                },
                Instruction {
                    on: true,
                    bounds: [[-39, 5], [-6, 47], [-3, 44]],
                },
                Instruction {
                    on: true,
                    bounds: [[-30, 21], [-8, 43], [-13, 34]],
                },
                Instruction {
                    on: true,
                    bounds: [[-22, 26], [-27, 20], [-29, 19]],
                },
                Instruction {
                    on: false,
                    bounds: [[-48, -32], [26, 41], [-47, -37]],
                },
                Instruction {
                    on: true,
                    bounds: [[-12, 35], [6, 50], [-50, -2]],
                },
                Instruction {
                    on: false,
                    bounds: [[-48, -32], [-32, -16], [-15, -5]],
                },
                Instruction {
                    on: true,
                    bounds: [[-18, 26], [-33, 15], [-7, 46]],
                },
                Instruction {
                    on: false,
                    bounds: [[-40, -22], [-38, -28], [23, 41]],
                },
                Instruction {
                    on: true,
                    bounds: [[-16, 35], [-41, 10], [-47, 6]],
                },
                Instruction {
                    on: false,
                    bounds: [[-32, -23], [11, 30], [-14, 3]],
                },
                Instruction {
                    on: true,
                    bounds: [[-49, -5], [-3, 45], [-29, 18]],
                },
                Instruction {
                    on: false,
                    bounds: [[18, 30], [-20, -8], [-3, 13]],
                },
                Instruction {
                    on: true,
                    bounds: [[-41, 9], [-7, 43], [-33, 15]],
                },
                Instruction {
                    on: true,
                    bounds: [[-54112, -39298], [-85059, -49293], [-27449, 7877]],
                },
                Instruction {
                    on: true,
                    bounds: [[967, 23432], [45373, 81175], [27513, 53682]],
                },
            ],
            min_x: -54112,
            max_x: 23432,
            min_y: -85059,
            max_y: 81175,
            min_z: -27449,
            max_z: 53682,
        }
    }

    fn extra_large_example_data() -> Input {
        Input {
            instructions: vec![
                Instruction {
                    on: true,
                    bounds: [[-5, 47], [-31, 22], [-19, 33]],
                },
                Instruction {
                    on: true,
                    bounds: [[-44, 5], [-27, 21], [-14, 35]],
                },
                Instruction {
                    on: true,
                    bounds: [[-49, -1], [-11, 42], [-10, 38]],
                },
                Instruction {
                    on: true,
                    bounds: [[-20, 34], [-40, 6], [-44, 1]],
                },
                Instruction {
                    on: false,
                    bounds: [[26, 39], [40, 50], [-2, 11]],
                },
                Instruction {
                    on: true,
                    bounds: [[-41, 5], [-41, 6], [-36, 8]],
                },
                Instruction {
                    on: false,
                    bounds: [[-43, -33], [-45, -28], [7, 25]],
                },
                Instruction {
                    on: true,
                    bounds: [[-33, 15], [-32, 19], [-34, 11]],
                },
                Instruction {
                    on: false,
                    bounds: [[35, 47], [-46, -34], [-11, 5]],
                },
                Instruction {
                    on: true,
                    bounds: [[-14, 36], [-6, 44], [-16, 29]],
                },
                Instruction {
                    on: true,
                    bounds: [[-57795, -6158], [29564, 72030], [20435, 90618]],
                },
                Instruction {
                    on: true,
                    bounds: [[36731, 105352], [-21140, 28532], [16094, 90401]],
                },
                Instruction {
                    on: true,
                    bounds: [[30999, 107136], [-53464, 15513], [8553, 71215]],
                },
                Instruction {
                    on: true,
                    bounds: [[13528, 83982], [-99403, -27377], [-24141, 23996]],
                },
                Instruction {
                    on: true,
                    bounds: [[-72682, -12347], [18159, 111354], [7391, 80950]],
                },
                Instruction {
                    on: true,
                    bounds: [[-1060, 80757], [-65301, -20884], [-103788, -16709]],
                },
                Instruction {
                    on: true,
                    bounds: [[-83015, -9461], [-72160, -8347], [-81239, -26856]],
                },
                Instruction {
                    on: true,
                    bounds: [[-52752, 22273], [-49450, 9096], [54442, 119054]],
                },
                Instruction {
                    on: true,
                    bounds: [[-29982, 40483], [-108474, -28371], [-24328, 38471]],
                },
                Instruction {
                    on: true,
                    bounds: [[-4958, 62750], [40422, 118853], [-7672, 65583]],
                },
                Instruction {
                    on: true,
                    bounds: [[55694, 108686], [-43367, 46958], [-26781, 48729]],
                },
                Instruction {
                    on: true,
                    bounds: [[-98497, -18186], [-63569, 3412], [1232, 88485]],
                },
                Instruction {
                    on: true,
                    bounds: [[-726, 56291], [-62629, 13224], [18033, 85226]],
                },
                Instruction {
                    on: true,
                    bounds: [[-110886, -34664], [-81338, -8658], [8914, 63723]],
                },
                Instruction {
                    on: true,
                    bounds: [[-55829, 24974], [-16897, 54165], [-121762, -28058]],
                },
                Instruction {
                    on: true,
                    bounds: [[-65152, -11147], [22489, 91432], [-58782, 1780]],
                },
                Instruction {
                    on: true,
                    bounds: [[-120100, -32970], [-46592, 27473], [-11695, 61039]],
                },
                Instruction {
                    on: true,
                    bounds: [[-18631, 37533], [-124565, -50804], [-35667, 28308]],
                },
                Instruction {
                    on: true,
                    bounds: [[-57817, 18248], [49321, 117703], [5745, 55881]],
                },
                Instruction {
                    on: true,
                    bounds: [[14781, 98692], [-1341, 70827], [15753, 70151]],
                },
                Instruction {
                    on: true,
                    bounds: [[-34419, 55919], [-19626, 40991], [39015, 114138]],
                },
                Instruction {
                    on: true,
                    bounds: [[-60785, 11593], [-56135, 2999], [-95368, -26915]],
                },
                Instruction {
                    on: true,
                    bounds: [[-32178, 58085], [17647, 101866], [-91405, -8878]],
                },
                Instruction {
                    on: true,
                    bounds: [[-53655, 12091], [50097, 105568], [-75335, -4862]],
                },
                Instruction {
                    on: true,
                    bounds: [[-111166, -40997], [-71714, 2688], [5609, 50954]],
                },
                Instruction {
                    on: true,
                    bounds: [[-16602, 70118], [-98693, -44401], [5197, 76897]],
                },
                Instruction {
                    on: true,
                    bounds: [[16383, 101554], [4615, 83635], [-44907, 18747]],
                },
                Instruction {
                    on: false,
                    bounds: [[-95822, -15171], [-19987, 48940], [10804, 104439]],
                },
                Instruction {
                    on: true,
                    bounds: [[-89813, -14614], [16069, 88491], [-3297, 45228]],
                },
                Instruction {
                    on: true,
                    bounds: [[41075, 99376], [-20427, 49978], [-52012, 13762]],
                },
                Instruction {
                    on: true,
                    bounds: [[-21330, 50085], [-17944, 62733], [-112280, -30197]],
                },
                Instruction {
                    on: true,
                    bounds: [[-16478, 35915], [36008, 118594], [-7885, 47086]],
                },
                Instruction {
                    on: false,
                    bounds: [[-98156, -27851], [-49952, 43171], [-99005, -8456]],
                },
                Instruction {
                    on: false,
                    bounds: [[2032, 69770], [-71013, 4824], [7471, 94418]],
                },
                Instruction {
                    on: true,
                    bounds: [[43670, 120875], [-42068, 12382], [-24787, 38892]],
                },
                Instruction {
                    on: false,
                    bounds: [[37514, 111226], [-45862, 25743], [-16714, 54663]],
                },
                Instruction {
                    on: false,
                    bounds: [[25699, 97951], [-30668, 59918], [-15349, 69697]],
                },
                Instruction {
                    on: false,
                    bounds: [[-44271, 17935], [-9516, 60759], [49131, 112598]],
                },
                Instruction {
                    on: true,
                    bounds: [[-61695, -5813], [40978, 94975], [8655, 80240]],
                },
                Instruction {
                    on: false,
                    bounds: [[-101086, -9439], [-7088, 67543], [33935, 83858]],
                },
                Instruction {
                    on: false,
                    bounds: [[18020, 114017], [-48931, 32606], [21474, 89843]],
                },
                Instruction {
                    on: false,
                    bounds: [[-77139, 10506], [-89994, -18797], [-80, 59318]],
                },
                Instruction {
                    on: false,
                    bounds: [[8476, 79288], [-75520, 11602], [-96624, -24783]],
                },
                Instruction {
                    on: true,
                    bounds: [[-47488, -1262], [24338, 100707], [16292, 72967]],
                },
                Instruction {
                    on: false,
                    bounds: [[-84341, 13987], [2429, 92914], [-90671, -1318]],
                },
                Instruction {
                    on: false,
                    bounds: [[-37810, 49457], [-71013, -7894], [-105357, -13188]],
                },
                Instruction {
                    on: false,
                    bounds: [[-27365, 46395], [31009, 98017], [15428, 76570]],
                },
                Instruction {
                    on: false,
                    bounds: [[-70369, -16548], [22648, 78696], [-1892, 86821]],
                },
                Instruction {
                    on: true,
                    bounds: [[-53470, 21291], [-120233, -33476], [-44150, 38147]],
                },
                Instruction {
                    on: false,
                    bounds: [[-93533, -4276], [-16170, 68771], [-104985, -24507]],
                },
            ],
            min_x: -120100,
            max_x: 120875,
            min_y: -124565,
            max_y: 118853,
            min_z: -121762,
            max_z: 119054,
        }
    }
}
