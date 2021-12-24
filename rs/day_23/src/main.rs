#![deny(clippy::all)]
#![feature(test)]
#![feature(available_parallelism)]
extern crate test;

use std::{
    collections::HashMap,
    fs::read_to_string,
    ops::AddAssign,
    path::Path,
    sync::{Arc, Mutex, RwLock},
    thread,
};

type Input = [Amphipod; 15];

fn main() {
    let data = read_data("./data");

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

fn read_data(data_dir: &str) -> Input {
    let contents = read_to_string(Path::new(data_dir).join("day_23.txt")).unwrap();

    parse_contents(contents.trim())
}

fn parse_contents(contents: &str) -> Input {
    let mut output = Input::default();
    contents
        .lines()
        .skip(2)
        .take(2)
        .enumerate()
        .for_each(|(line_idx, line)| {
            line.trim_start_matches(&[' ', '#'][..])
                .split('#')
                .take(4)
                .enumerate()
                .for_each(|(idx, amphipod)| {
                    output[7 + (2 * idx) + line_idx] = match amphipod {
                        "A" => Amphipod::Amber,
                        "B" => Amphipod::Bronze,
                        "C" => Amphipod::Copper,
                        "D" => Amphipod::Desert,
                        _ => panic!("invalid amphipod character: '{}'", amphipod),
                    };
                });
        });
    output
}

fn part_1(input: &Input) -> usize {
    let target_hash = hash([
        Amphipod::Empty,
        Amphipod::Empty,
        Amphipod::Empty,
        Amphipod::Empty,
        Amphipod::Empty,
        Amphipod::Empty,
        Amphipod::Empty,
        Amphipod::Amber,
        Amphipod::Amber,
        Amphipod::Bronze,
        Amphipod::Bronze,
        Amphipod::Copper,
        Amphipod::Copper,
        Amphipod::Desert,
        Amphipod::Desert,
    ]);

    let input_hash = hash(*input);

    let mut energy_totals = HashMap::<u64, usize>::new();
    energy_totals.insert(input_hash, 0);
    let orig_energy_lock = Arc::new(RwLock::new(energy_totals));

    let mut to_process = Vec::with_capacity(100_000);
    to_process.push((*input, input_hash));
    let orig_process_lock = Arc::new(Mutex::new(to_process));

    let result = Arc::new(RwLock::new(0));

    let mut thread_pool = Vec::new();
    for _ in 0..2 {
        let res = Arc::clone(&result);
        let energy_lock = Arc::clone(&orig_energy_lock);
        let process_lock = Arc::clone(&orig_process_lock);
        let handle = thread::spawn(move || {
            // let mut iter = 0;
            while let Some((initial_state, initial_state_hash)) = {
                let mut l = process_lock.lock().unwrap();
                l.pop()
            } {
                if *res.read().unwrap() > 0 {
                    return;
                }
                let initial_energy = *energy_lock
                    .read()
                    .unwrap()
                    .get(&initial_state_hash)
                    .unwrap();

                // assert!(initial_energy < 13_000);
                if initial_state_hash == target_hash {
                    *res.write().unwrap() = initial_energy;
                    return;
                }

                let (to_range, from_range) =
                    initial_state
                        .iter()
                        .enumerate()
                        .partition::<Vec<(usize, &Amphipod)>, _>(|(_, &s)| s == Amphipod::Empty);

                for (from, &from_am) in from_range {
                    // check state[from] can move
                    let energy = match from_am {
                        Amphipod::Empty => continue,
                        Amphipod::Amber => 1,
                        Amphipod::Bronze => 10,
                        Amphipod::Copper => 100,
                        Amphipod::Desert => 1000,
                    };
                    for &(to, _) in &to_range {
                        let steps = match can_move(initial_state, from, to) {
                            Some(s) => s,
                            None => continue,
                        };

                        let mut new_state = initial_state;
                        new_state[to] = from_am;
                        new_state[from] = Amphipod::Empty;

                        let new_energy =
                            initial_energy + (steps * energy) + out_of_place_wt(new_state);
                        let new_state_hash = hash(new_state);

                        if new_energy.lt(energy_lock
                            .read()
                            .unwrap()
                            .get(&new_state_hash)
                            .unwrap_or(&usize::MAX))
                        {
                            energy_lock
                                .write()
                                .unwrap()
                                .insert(new_state_hash, new_energy);
                            process_lock
                                .lock()
                                .unwrap()
                                .push((new_state, new_state_hash));
                        }
                    }
                }

                if *res.read().unwrap() > 0 {
                    return;
                }

                let totals = energy_lock.read().unwrap();
                let mut l = process_lock.lock().unwrap();
                l.sort_by_cached_key(|(_, a)| totals.get(a).unwrap_or(&usize::MAX));
                l.reverse();
            }
        });
        thread_pool.push(handle);
    }

    for t in thread_pool {
        t.join().unwrap();
    }

    let r = result.read().unwrap();
    *r
}

fn part_2(input: &Input) -> usize {
    todo!()
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
#[repr(u8)]
enum Amphipod {
    Empty,
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Default for Amphipod {
    fn default() -> Self {
        Amphipod::Empty
    }
}

fn hash(amphipods: Input) -> u64 {
    amphipods
        .iter()
        .fold(0u64, |acc, amphipod| acc << 3 | *amphipod as u64)
}

// fn unhash(h: u64) -> Input {
//     let mut out = Input::default();
//     let out_len = out.len();
//     for (i, amphipod) in out.iter_mut().enumerate() {
//         let val = h >> (3 * (out_len - i - 1)) & 0b111;
//         *amphipod = match val {
//             1 => Amphipod::Amber,
//             2 => Amphipod::Bronze,
//             3 => Amphipod::Copper,
//             4 => Amphipod::Desert,
//             _ => Amphipod::Empty,
//         };
//     }
//     out
// }

fn out_of_place_wt(state: Input) -> usize {
    let mut offset = 0;
    if state[7] != Amphipod::Amber {
        offset += 25;
    }
    if state[11] != Amphipod::Amber {
        offset += 50;
    }
    if state[8] != Amphipod::Bronze {
        offset += 250;
    }
    if state[12] != Amphipod::Bronze {
        offset += 500;
    }
    if state[9] != Amphipod::Copper {
        offset += 2500;
    }
    if state[13] != Amphipod::Copper {
        offset += 5000;
    }
    if state[10] != Amphipod::Desert {
        offset += 25000;
    }
    if state[14] != Amphipod::Desert {
        offset += 50000;
    }
    offset
}

#[inline]
fn can_move(state: Input, from: usize, to: usize) -> Option<usize> {
    match from {
        0 => can_move_from_0(state, to),
        1 => can_move_from_1(state, to),
        2 => can_move_from_2(state, to),
        3 => can_move_from_3(state, to),
        4 => can_move_from_4(state, to),
        5 => can_move_from_5(state, to),
        6 => can_move_from_6(state, to),
        7 => can_move_from_7(state, to),
        8 => can_move_from_8(state, to),
        9 => can_move_from_9(state, to),
        10 => can_move_from_10(state, to),
        11 => can_move_from_11(state, to),
        12 => can_move_from_12(state, to),
        13 => can_move_from_13(state, to),
        14 => can_move_from_14(state, to),
        _ => None,
    }
}

fn can_move_from_0(state: Input, to: usize) -> Option<usize> {
    match to {
        0..=6 => None,
        7 => {
            let steps = [1];
            if state[11] != state[0] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        8 => {
            let steps = [1, 2];
            if state[12] != state[0] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        9 => {
            let steps = [1, 2, 3];
            if state[13] != state[0] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        10 => {
            let steps = [1, 2, 3, 4];
            if state[14] != state[0] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        11 => {
            let steps = [1, 7];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        12 => {
            let steps = [1, 2, 8];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        13 => {
            let steps = [1, 2, 3, 9];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        14 => {
            let steps = [1, 2, 3, 4, 10];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        _ => None,
    }
}

fn can_move_from_1(state: Input, to: usize) -> Option<usize> {
    match to {
        0..=6 => None,
        7 => {
            if state[11] != state[1] {
                None
            } else {
                Some(2)
            }
        }
        8 => {
            let steps = [2];
            if state[12] != state[1] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        9 => {
            let steps = [2, 3];
            if state[13] != state[1] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        10 => {
            let steps = [2, 3, 4];
            if state[14] != state[1] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        11 => {
            let steps = [7];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        12 => {
            let steps = [2, 8];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        13 => {
            let steps = [2, 3, 9];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        14 => {
            let steps = [2, 3, 4, 10];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        _ => None,
    }
}

fn can_move_from_2(state: Input, to: usize) -> Option<usize> {
    match to {
        0..=6 => None,
        7 => {
            if state[11] != state[2] {
                None
            } else {
                Some(2)
            }
        }
        8 => {
            if state[12] != state[2] {
                None
            } else {
                Some(2)
            }
        }
        9 => {
            let steps = [3];
            if state[13] != state[2] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        10 => {
            let steps = [3, 4];
            if state[14] != state[2] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        11 => {
            let steps = [7];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        12 => {
            let steps = [8];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        13 => {
            let steps = [3, 9];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        14 => {
            let steps = [3, 4, 10];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        _ => None,
    }
}

fn can_move_from_3(state: Input, to: usize) -> Option<usize> {
    match to {
        0..=6 => None,
        7 => {
            let steps = [2];
            if state[11] != state[3] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        8 => {
            if state[12] != state[3] {
                None
            } else {
                Some(2)
            }
        }
        9 => {
            if state[13] != state[3] {
                None
            } else {
                Some(2)
            }
        }
        10 => {
            let steps = [4];
            if state[14] != state[3] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        11 => {
            let steps = [2, 7];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        12 => {
            let steps = [8];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        13 => {
            let steps = [9];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        14 => {
            let steps = [4, 10];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        _ => None,
    }
}

fn can_move_from_4(state: Input, to: usize) -> Option<usize> {
    match to {
        0..=6 => None,
        7 => {
            let steps = [3, 2];
            if state[11] != state[4] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        8 => {
            let steps = [3];
            if state[12] != state[4] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        9 => {
            if state[13] != state[4] {
                None
            } else {
                Some(2)
            }
        }
        10 => {
            if state[14] != state[4] {
                None
            } else {
                Some(2)
            }
        }
        11 => {
            let steps = [3, 2, 7];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        12 => {
            let steps = [3, 8];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        13 => {
            let steps = [9];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        14 => {
            let steps = [10];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        _ => None,
    }
}

fn can_move_from_5(state: Input, to: usize) -> Option<usize> {
    match to {
        0..=6 => None,
        7 => {
            let steps = [4, 3, 2];
            if state[11] != state[5] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        8 => {
            let steps = [4, 3];
            if state[12] != state[5] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        9 => {
            let steps = [4];
            if state[13] != state[5] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        10 => {
            if state[14] != state[5] {
                None
            } else {
                Some(2)
            }
        }
        11 => {
            let steps = [4, 3, 2, 7];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        12 => {
            let steps = [4, 3, 8];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        13 => {
            let steps = [4, 9];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        14 => {
            let steps = [10];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        _ => None,
    }
}

fn can_move_from_6(state: Input, to: usize) -> Option<usize> {
    match to {
        0..=6 => None,
        7 => {
            let steps = [5, 4, 3, 2];
            if state[11] != state[6] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        8 => {
            let steps = [5, 4, 3];
            if state[12] != state[6] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        9 => {
            let steps = [5, 4];
            if state[13] != state[6] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        10 => {
            let steps = [5];
            if state[14] != state[6] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        11 => {
            let steps = [5, 4, 3, 2, 7];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        12 => {
            let steps = [5, 4, 3, 8];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        13 => {
            let steps = [5, 4, 9];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        14 => {
            let steps = [5, 10];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        _ => None,
    }
}

fn can_move_from_7(state: Input, to: usize) -> Option<usize> {
    if state[7] == Amphipod::Amber && state[11] == Amphipod::Amber {
        return None;
    }
    match to {
        0 => {
            let steps = [1];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        1 => Some(2),
        2 => Some(2),
        3 => {
            let steps = [2];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        4 => {
            let steps = [2, 3];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        5 => {
            let steps = [2, 3, 4];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        6 => {
            let steps = [2, 3, 4, 5];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        7 => None,
        8 => {
            let steps = [2];
            if state[12] != state[7] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        9 => {
            let steps = [2, 3];
            if state[13] != state[7] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        10 => {
            let steps = [2, 3, 4];
            if state[14] != state[7] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        11 => None,
        12 => {
            let steps = [2, 8];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        13 => {
            let steps = [2, 3, 9];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        14 => {
            let steps = [2, 3, 4, 10];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        _ => None,
    }
}

fn can_move_from_8(state: Input, to: usize) -> Option<usize> {
    if state[8] == Amphipod::Bronze && state[12] == Amphipod::Bronze {
        return None;
    }
    match to {
        0 => {
            let steps = [2, 1];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        1 => {
            let steps = [2];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        2 => Some(2),
        3 => Some(2),
        4 => {
            let steps = [3];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        5 => {
            let steps = [3, 4];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        6 => {
            let steps = [3, 4, 5];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        7 => {
            let steps = [2];
            if state[11] != state[8] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        8 => None,
        9 => {
            let steps = [3];
            if state[13] != state[8] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        10 => {
            let steps = [3, 4];
            if state[14] != state[8] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        11 => {
            let steps = [2, 7];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        12 => None,
        13 => {
            let steps = [3, 9];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        14 => {
            let steps = [3, 4, 10];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        _ => None,
    }
}

fn can_move_from_9(state: Input, to: usize) -> Option<usize> {
    if state[9] == Amphipod::Copper && state[13] == Amphipod::Copper {
        return None;
    }
    match to {
        0 => {
            let steps = [3, 2, 1];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        1 => {
            let steps = [3, 2];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        2 => {
            let steps = [3];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        3 => Some(2),
        4 => Some(2),
        5 => {
            let steps = [4];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        6 => {
            let steps = [4, 5];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        7 => {
            let steps = [3, 2];
            if state[11] != state[9] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        8 => {
            let steps = [3];
            if state[12] != state[9] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        9 => None,
        10 => {
            let steps = [4];
            if state[14] != state[9] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        11 => {
            let steps = [3, 2, 7];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        12 => {
            let steps = [3, 8];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        13 => None,
        14 => {
            let steps = [4, 10];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        _ => None,
    }
}

fn can_move_from_10(state: Input, to: usize) -> Option<usize> {
    if state[10] == Amphipod::Desert && state[14] == Amphipod::Desert {
        return None;
    }
    match to {
        0 => {
            let steps = [4, 3, 2, 1];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        1 => {
            let steps = [4, 3, 2];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        2 => {
            let steps = [4, 3];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        3 => {
            let steps = [4];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        4 => Some(2),
        5 => Some(2),
        6 => {
            let steps = [5];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        7 => {
            let steps = [4, 3, 2];
            if state[11] != state[10] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        8 => {
            let steps = [4, 3];
            if state[12] != state[10] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        9 => {
            let steps = [4];
            if state[13] != state[10] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        10 => None,
        11 => {
            let steps = [4, 3, 2, 7];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        12 => {
            let steps = [4, 3, 8];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        13 => {
            let steps = [4, 9];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        14 => None,
        _ => None,
    }
}

fn can_move_from_11(state: Input, to: usize) -> Option<usize> {
    if state[11] == Amphipod::Amber {
        return None;
    }
    match to {
        0 => {
            let steps = [7, 1];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        1 => {
            let steps = [7];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        2 => {
            let steps = [7];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        3 => {
            let steps = [7, 2];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        4 => {
            let steps = [7, 2, 3];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        5 => {
            let steps = [7, 2, 3, 4];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        6 => {
            let steps = [7, 2, 3, 4, 5];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        7 => None,
        8 => {
            let steps = [7, 2];
            if state[12] != state[11] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        9 => {
            let steps = [7, 2, 3];
            if state[13] != state[11] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        10 => {
            let steps = [7, 2, 3, 4];
            if state[14] != state[11] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        11 => None,
        12 => {
            let steps = [7, 2, 8];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        13 => {
            let steps = [7, 2, 3, 9];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        14 => {
            let steps = [7, 2, 3, 4, 10];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        _ => None,
    }
}

fn can_move_from_12(state: Input, to: usize) -> Option<usize> {
    if state[12] == Amphipod::Bronze {
        return None;
    }
    match to {
        0 => {
            let steps = [8, 2, 1];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        1 => {
            let steps = [8, 2];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        2 => {
            let steps = [8];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        3 => {
            let steps = [8];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        4 => {
            let steps = [8, 3];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        5 => {
            let steps = [8, 3, 4];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        6 => {
            let steps = [8, 3, 4, 5];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        7 => {
            let steps = [8, 2];
            if state[11] != state[12] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        8 => None,
        9 => {
            let steps = [8, 3];
            if state[13] != state[12] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        10 => {
            let steps = [8, 3, 4];
            if state[14] != state[12] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        11 => {
            let steps = [8, 2, 7];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        12 => None,
        13 => {
            let steps = [8, 3, 9];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        14 => {
            let steps = [8, 3, 4, 10];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        _ => None,
    }
}

fn can_move_from_13(state: Input, to: usize) -> Option<usize> {
    if state[13] == Amphipod::Copper {
        return None;
    }
    match to {
        0 => {
            let steps = [9, 3, 2, 1];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        1 => {
            let steps = [9, 3, 2];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        2 => {
            let steps = [9, 3];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        3 => {
            let steps = [9];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        4 => {
            let steps = [9];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        5 => {
            let steps = [9, 4];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        6 => {
            let steps = [9, 4, 5];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        7 => {
            let steps = [9, 3, 2];
            if state[11] != state[13] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        8 => {
            let steps = [9, 3];
            if state[12] != state[13] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        9 => None,
        10 => {
            let steps = [9, 4];
            if state[14] != state[13] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        11 => {
            let steps = [9, 3, 2, 7];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        12 => {
            let steps = [9, 3, 8];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        13 => None,
        14 => {
            let steps = [9, 4, 10];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        _ => None,
    }
}

fn can_move_from_14(state: Input, to: usize) -> Option<usize> {
    if state[14] == Amphipod::Desert {
        return None;
    }
    match to {
        0 => {
            let steps = [10, 4, 3, 2, 1];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        1 => {
            let steps = [10, 4, 3, 2];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        2 => {
            let steps = [10, 4, 3];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        3 => {
            let steps = [10, 4];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        4 => {
            let steps = [10];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        5 => {
            let steps = [10];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        6 => {
            let steps = [10, 5];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 2)
            }
        }
        7 => {
            let steps = [10, 4, 3, 2];
            if state[11] != state[14] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        8 => {
            let steps = [10, 4, 3];
            if state[12] != state[14] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        9 => {
            let steps = [10, 4];
            if state[13] != state[14] || steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        10 => None,
        11 => {
            let steps = [10, 4, 3, 2, 7];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 5)
            }
        }
        12 => {
            let steps = [10, 4, 3, 8];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 4)
            }
        }
        13 => {
            let steps = [10, 4, 9];
            if steps.iter().any(|&idx| state[idx] != Amphipod::Empty) {
                None
            } else {
                Some(steps.len() + 3)
            }
        }
        14 => None,
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const PART_1: usize = 12521;
    const PART_2: usize = 0;

    // mod unhash {
    //     use super::*;

    //     #[test]
    //     fn foo() {
    //         let mut input = Input::default();
    //         input[4] = Amphipod::Copper;
    //         input[7] = Amphipod::Amber;
    //         input[13] = Amphipod::Desert;

    //         assert_eq!(input, unhash(hash(input)));
    //     }
    // }

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
                input: "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########",
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
                expected: 12521,
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
        [
            Amphipod::Empty,
            Amphipod::Empty,
            Amphipod::Empty,
            Amphipod::Empty,
            Amphipod::Empty,
            Amphipod::Empty,
            Amphipod::Empty,
            Amphipod::Bronze,
            Amphipod::Amber,
            Amphipod::Copper,
            Amphipod::Desert,
            Amphipod::Bronze,
            Amphipod::Copper,
            Amphipod::Desert,
            Amphipod::Amber,
        ]
    }
}
