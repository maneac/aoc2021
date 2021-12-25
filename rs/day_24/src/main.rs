#![deny(clippy::all)]
#![feature(test)]
extern crate test;

use std::{fs::read_to_string, path::Path, str::FromStr, sync::mpsc, thread};

type Input = Vec<Instruction>;

fn main() {
    let data = read_data("./data");

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

fn read_data(data_dir: &str) -> Input {
    let contents = read_to_string(Path::new(data_dir).join("day_24.txt")).unwrap();

    parse_contents(contents.trim())
}

fn parse_contents(contents: &str) -> Input {
    contents.lines().fold(Vec::new(), |mut acc, line| {
        acc.push(Instruction::from_str(line).unwrap());
        acc
    })
}

fn part_1_brute(input: &Input) -> usize {
    let (send, recv) = mpsc::channel();

    let mut threads = Vec::with_capacity(10);
    for i in 1..=9 {
        let instructions = input.clone();
        let result_chan = send.clone();
        let handle = thread::spawn(move || {
            for model_number in (ModelNumber {
                num: [i, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
            }) {
                let mut l = LogicUnit::new(model_number);
                for &instruction in &instructions {
                    l.run(instruction);
                }
                if l.is_valid() {
                    result_chan.send(l.into()).unwrap();
                    return;
                }
            }
        });
        threads.push(handle);
    }

    let mut results = Vec::new();

    while let Ok(r) = recv.recv() {
        results.push(r);
    }

    for h in threads {
        h.join().unwrap();
    }

    *results.iter().max().unwrap()
}

fn part_2(input: &Input) -> usize {
    todo!()
}

struct ModelNumber {
    num: [u8; 14],
}

impl Iterator for ModelNumber {
    type Item = [u8; 14];

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.num;
        for i in (0..13).rev() {
            if self.num[i] > 1 {
                self.num[i] -= 1;
                return Some(out);
            }
            self.num[i] = 9;
        }
        None
    }
}

impl ModelNumber {
    fn new() -> Self {
        Self { num: [9u8; 14] }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
struct LogicUnit {
    registers: [isize; 4],
    model_number: [u8; 14],
    input_idx: usize,
}

impl LogicUnit {
    fn new(model_number: [u8; 14]) -> Self {
        Self {
            registers: [0isize; 4],
            model_number,
            input_idx: 13,
        }
    }

    fn run(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => {}
            Instruction::Input(register) => {
                self.registers[register as usize] = self.model_number[self.input_idx] as isize;
                self.input_idx -= 1;
            }
            Instruction::Add(a, VarNum::N(b)) => self.registers[a as usize] += b,
            Instruction::Add(a, VarNum::V(b)) => {
                self.registers[a as usize] += self.registers[b as usize]
            }
            Instruction::Mul(a, VarNum::N(b)) => self.registers[a as usize] *= b,
            Instruction::Mul(a, VarNum::V(b)) => {
                self.registers[a as usize] *= self.registers[b as usize]
            }
            Instruction::Div(a, VarNum::N(b)) => self.registers[a as usize] /= b,
            Instruction::Div(a, VarNum::V(b)) => {
                self.registers[a as usize] /= self.registers[b as usize]
            }
            Instruction::Mod(a, VarNum::N(b)) => self.registers[a as usize] %= b,
            Instruction::Mod(a, VarNum::V(b)) => {
                self.registers[a as usize] %= self.registers[b as usize]
            }
            Instruction::Eql(a, VarNum::N(b)) => {
                self.registers[a as usize] = (self.registers[a as usize] == b) as isize
            }
            Instruction::Eql(a, VarNum::V(b)) => {
                self.registers[a as usize] =
                    (self.registers[a as usize] == self.registers[b as usize]) as isize
            }
        }
    }

    fn is_valid(&self) -> bool {
        self.registers[3] == 0
    }
}

impl From<usize> for LogicUnit {
    fn from(model_number: usize) -> Self {
        let mut num = [0u8; 14];
        format!("{}", model_number)
            .bytes()
            .zip(num.iter_mut())
            .for_each(|(m, n)| *n = m - b'0');
        Self::new(num)
    }
}

impl From<LogicUnit> for usize {
    fn from(unit: LogicUnit) -> Self {
        unit.model_number
            .iter()
            .fold(0usize, |acc, &n| (acc * 10) + n as usize)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Instruction {
    Noop,
    Input(Var),
    Add(Var, VarNum),
    Mul(Var, VarNum),
    Div(Var, VarNum),
    Mod(Var, VarNum),
    Eql(Var, VarNum),
}

impl Default for Instruction {
    fn default() -> Self {
        Self::Noop
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();

        let keyword = parts
            .next()
            .ok_or("cannot convert an empty string to an Instruction")?;

        let var = Var::from_str(parts.next().ok_or("no variable for instruction")?)?;

        let var_num = if let Some(s) = parts.next() {
            VarNum::from_str(s)
        } else {
            Err("no second variable or number for instruction".to_string())
        };

        match keyword {
            "inp" => Ok(Instruction::Input(var)),
            "add" => Ok(Instruction::Add(var, var_num?)),
            "mul" => Ok(Instruction::Mul(var, var_num?)),
            "div" => Ok(Instruction::Div(var, var_num?)),
            "mod" => Ok(Instruction::Mod(var, var_num?)),
            "eql" => Ok(Instruction::Eql(var, var_num?)),
            _ => Err(format!("invalid instruction keyword: {}", keyword)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum VarNum {
    N(isize),
    V(Var),
}

impl FromStr for VarNum {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse::<isize>() {
            Ok(Self::N(num))
        } else {
            Ok(VarNum::V(Var::from_str(s)?))
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u8)]
enum Var {
    W,
    X,
    Y,
    Z,
}

impl From<u8> for Var {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::W,
            1 => Self::X,
            2 => Self::Y,
            3 => Self::Z,
            _ => panic!("invalid Var: {}", val),
        }
    }
}

impl FromStr for Var {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Self::W),
            "x" => Ok(Self::X),
            "y" => Ok(Self::Y),
            "z" => Ok(Self::Z),
            _ => Err(format!("invalid variable: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const PART_1: usize = 0;
    const PART_2: usize = 0;

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

    mod logic_unit {
        use super::*;

        #[test]
        fn new_unit() {
            let input = 9;

            let expected = LogicUnit {
                registers: [0isize; 4],
                model_number: [9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                input_idx: 13,
            };

            assert_eq!(expected, LogicUnit::from(input))
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
                input: "inp x
mul x -1",
                expected: example_1_data(),
            })
        }

        #[test]
        fn example_2() {
            run(&Case {
                input: "inp z
inp x
mul z 3
eql z x",
                expected: example_2_data(),
            })
        }

        #[test]
        fn example_3() {
            run(&Case {
                input: "inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2",
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
                expected: 99_999_999_999_999,
            })
        }

        #[test]
        fn example_2() {
            run(&Case {
                data: example_2_data(),
                expected: 99_999_999_999_999,
            })
        }

        #[test]
        fn example_3() {
            run(&Case {
                data: example_3_data(),
                expected: 99_999_999_999_998,
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
                expected: todo!(),
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
        todo!()
    }

    fn example_1_data() -> Input {
        vec![
            Instruction::Input(Var::X),
            Instruction::Mul(Var::X, VarNum::N(-1)),
        ]
    }

    fn example_2_data() -> Input {
        vec![
            Instruction::Input(Var::Z),
            Instruction::Input(Var::X),
            Instruction::Mul(Var::Z, VarNum::N(3)),
            Instruction::Eql(Var::Z, VarNum::V(Var::X)),
        ]
    }

    fn example_3_data() -> Input {
        vec![
            Instruction::Input(Var::W),
            Instruction::Add(Var::Z, VarNum::V(Var::W)),
            Instruction::Mod(Var::Z, VarNum::N(2)),
            Instruction::Div(Var::W, VarNum::N(2)),
            Instruction::Add(Var::Y, VarNum::V(Var::W)),
            Instruction::Mod(Var::Y, VarNum::N(2)),
            Instruction::Div(Var::W, VarNum::N(2)),
            Instruction::Add(Var::X, VarNum::V(Var::W)),
            Instruction::Mod(Var::X, VarNum::N(2)),
            Instruction::Div(Var::W, VarNum::N(2)),
            Instruction::Mod(Var::W, VarNum::N(2)),
        ]
    }
}

fn part_1(_: &Input) -> usize {
    let [a, b, c, d, e, f, g, h, i, j, k, l, m, n] = [0isize; 14];
    let [mut w, mut x, mut y, mut z] = [0isize; 4];

    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 1
    // add x 12
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    // mul y 0
    // add y w
    // add y 4
    // mul y x
    // add z y
    w = a;
    z = a + 4;

    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 1
    // add x 15
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    // mul y 0
    // add y w
    // add y 11
    // mul y x
    // add z y
    w = b;
    z *= 26;
    z += b + 11;

    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 1
    // add x 11
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    // mul y 0
    // add y w
    // add y 7
    // mul y x
    // add z y
    w = c;
    z *= 26;
    z += c + 7;

    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 26
    // add x -14
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    // mul y 0
    // add y w
    // add y 2
    // mul y x
    // add z y
    w = d;
    z /= 26;
    z *= (25 * (c - 7 != d) as isize) + 1;
    z += (d + 2) * (c - 7 != d) as isize;

    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 1
    // add x 12
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    // mul y 0
    // add y w
    // add y 11
    // mul y x
    // add z y
    w = e;
    z *= 26;
    z += e + 11;

    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 26
    // add x -10
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    // mul y 0
    // add y w
    // add y 13
    // mul y x
    // add z y
    w = f;
    z /= 26;
    z *= (25 * (e + 1 != f) as isize) + 1;
    z += (f + 13) * (e + 1 != f) as isize;

    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 1
    // add x 11
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    // mul y 0
    // add y w
    // add y 9
    // mul y x
    // add z y
    w = g;
    z *= 26;
    z += g + 9;

    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 1
    // add x 13
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    // mul y 0
    // add y w
    // add y 12
    // mul y x
    // add z y
    w = h;
    z *= 26;
    z += h + 12;

    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 26
    // add x -7
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    // mul y 0
    // add y w
    // add y 6
    // mul y x
    // add z y
    w = i;
    z /= 26;
    z *= (25 * (h + 5 != i) as isize) + 1;
    z += (i + 6) * (h + 5 != i) as isize;

    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 1
    // add x 10
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    // mul y 0
    // add y w
    // add y 2
    // mul y x
    // add z y
    w = j;
    z *= 26;
    z += j + 2;

    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 26
    // add x -2
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    // mul y 0
    // add y w
    // add y 11
    // mul y x
    // add z y
    w = k;
    z /= 26;
    z *= (25 * (j != k) as isize) + 1;
    z += (k + 11) * (j != k) as isize;

    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 26
    // add x -1
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    // mul y 0
    // add y w
    // add y 12
    // mul y x
    // add z y
    w = l;
    z /= 26;
    z *= 26;
    z += l + 12;

    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 26
    // add x -4
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    // mul y 0
    // add y w
    // add y 3
    // mul y x
    // add z y
    w = m;
    z /= 26;
    z *= (25 + (l + 8 != m) as isize) + 1;
    z += (m + 3) * (l + 8 != m) as isize;

    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 26
    // add x -12
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    // mul y 0
    // add y w
    // add y 13
    // mul y x
    // add z y
    w = n;
    z2 = (z / 26) * ((25 * (z % 26 - 12 != n) as isize) + 1)
        + (n + 13) * (z % 26 - 12 != n) as isize;

    /*


















    */
    z = a + 4;
    z *= 26;
    z += b + 11;
    // z *= 26;
    // z += c + 7;
    // z /= 26;

    z *= (25 * (c - 7 != d) as isize) + 1;
    z += (d + 2) * (c - 7 != d) as isize;

    // z *= 26;
    // z += e + 11;
    // z /= 26;

    z *= (25 * (e + 1 != f) as isize) + 1;
    z += (f + 13) * (e + 1 != f) as isize;

    z *= 26;
    z += g + 9;

    // z *= 26;
    // z += h + 12;
    // z /= 26;

    z *= (25 * (h + 5 != i) as isize) + 1;
    z += (i + 6) * (h + 5 != i) as isize;

    // z *= 26;
    // z += j + 2;
    // z /= 26;

    z *= (25 * (j != k) as isize) + 1;
    z += (k + 11) * (j != k) as isize;

    z /= 26;
    // z *= 26;
    // z += l + 12;
    // z /= 26;

    z *= (25 + (l + 8 != m) as isize) + 1;
    z += (m + 3) * (l + 8 != m) as isize;

    // z /= 26;
    // z *= 26;
    // z += n + 13;

    /*























    */

    let s1 = (26 * (a + 4)) + (b + 11);

    let s2 = if c - 7 != d { (26 * s1) + (d + 2) } else { s1 };

    let s3 = if e + 1 != f { (26 * s2) + (f + 13) } else { s2 };

    let s4 = if h + 5 != i {
        (26 * ((26 * s3) + (g + 9))) + (i + 6)
    } else {
        (26 * s3) + (g + 9)
    };

    let s5 = if j != k { (26 * s4) + (k + 11) } else { s4 };

    let s6 = if l + 8 != m {
        (26 * s5) + (m + 3)
    } else {
        s5 / 26
    };

    let z_total = if s6 % 26 - 12 != n {
        26 * (s6 / 26) + (n + 13)
    } else {
        s6 / 26
    };

    //
    let model_num = [a, b, c, d, e, f, g, h, i, j, k, l, m, s6 % 26 - 12];



    // 13 <= (26 * ((26 * ((26 * ((26 * s3) + (g + 9))) + (i + 6))) + (k + 11))) + (m + 3) <= 21
    // [
    //     a, 
    //     b, 
    //     c,
    //     d, 
    //     e, 
    //     f, 
    //     g, 
    //     h,
    //     i: !(h+5),
    //     j,
    //     k: !(j),
    //     l,
    //     m: !(l+8),
    //     n: !(l+8),
    // ]


    // 13 <= ((26 * ((26 * s3) + (g + 9))) + (i + 6)) <= 21
    // [
    //     a, 
    //     b, 
    //     c,
    //     d, 
    //     e, 
    //     f, 
    //     g, 
    //     h,
    //     i: !(h+5),
    //     j,
    //     k: !(j),
    //     l: 1,
    //     m: 9,
    //     n: i-6,
    // ]

    // 13 <= (26 * ((26 * ((26 * s3) + (g + 9))) + (i + 6))) + (m + 3) <= 21
    // [
    //     a, 
    //     b, 
    //     c,
    //     d, 
    //     e, 
    //     f, 
    //     g, 
    //     h,
    //     i: !(h+5),
    //     j,
    //     k: j,
    //     l,
    //     m: !(l+8),
    //     n: !(l+8),
    // ]


    // 13 <= (26 * s3) + (g + 9) <= 21
    // [
    //     a, 
    //     b, 
    //     c,
    //     d, 
    //     e, 
    //     f, 
    //     g, 
    //     h,
    //     i: !(h+5),
    //     j,
    //     k: j,
    //     l: 1,
    //     m: 9,
    //     n: g - 3,
    // ]

    // 13 <= (26 * ((26 * ((26 * s3) + (g + 9))) + (k + 11))) + (m + 3) <= 21
    // [
    //     a, 
    //     b, 
    //     c,
    //     d, 
    //     e, 
    //     f, 
    //     g, 
    //     h,
    //     i: h+5,
    //     j,
    //     k: !(j),
    //     l,
    //     m: !(l+8),
    //     n: !(l+8),
    // ]


    // 13 <= ((26 * s3) + (g + 9)) <= 21
    // [
    //     a, 
    //     b, 
    //     c,
    //     d, 
    //     e, 
    //     f, 
    //     g, 
    //     h,
    //     i: h+5,
    //     j,
    //     k: !(j),
    //     l: 1,
    //     m: 9,
    //     n: g -3,
    // ]

    // 13 <= (26*((26 * s3) + (g + 9))) + (m + 3) <= 21
    // [
    //     a, 
    //     b, 
    //     c,
    //     d, 
    //     e, 
    //     f, 
    //     g, 
    //     h,
    //     i: h+5,
    //     j,
    //     k: j,
    //     l,
    //     m: !(l+8),
    //     n: !(l+8),
    // ]


    13 <=  ( (26 * s2) + (f + 13) )  <= 21
    [
        a, 
        b, 
        c,
        d, 
        e, 
        f: !(e+1), 
        g, 
        h,
        i: h+5,
        j,
        k: j,
        l: 1,
        m: 9,
        n: ( ( (26 * s2) + (f + 13) )  % 26) - 12,
    ]

    // // //

    13 <= s2 <= 21
    [
        a, 
        b, 
        c,
        d, 
        e, 
        f: e+1, 
        g, 
        h,
        i: h+5,
        j,
        k: j,
        l: 1,
        m: 9,
        n: (s2 % 26) - 12,
    ]
    
    0
}
