#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]
use regex::Regex;
use std::{
    env::args,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    time::Duration,
};
use thousands::Separable;

const EMPTY_STR: String = String::new();

fn main() {
    let day = if args().len() > 1 {
        args().nth(1).unwrap().parse::<usize>().unwrap()
    } else {
        0
    };

    let rust_results = bench_rust(day);
    let go_results = bench_go(day);
    let deno_results = bench_deno(day);

    let mut diff_table = String::new();

    let mut rust_table = String::new();
    let mut go_table = String::new();
    let mut deno_table = String::new();

    go_results
        .iter()
        .zip(rust_results)
        .zip(deno_results)
        .enumerate()
        .filter(|(_, ((go, rs), ts))| {
            [go, rs, ts]
                .iter()
                .any(|times| times.iter().any(|t| !t.is_zero()))
        })
        .for_each(|(day, ((go, rs), ts))| {
            let day = day + 1;
            println!("Day {}\n", day);

            let mut diffs = [[EMPTY_STR; 4], [EMPTY_STR; 4], [EMPTY_STR; 4]];

            let results = go.iter().zip(rs).zip(ts).enumerate().fold(
                [[EMPTY_STR; 4], [EMPTY_STR; 4], [EMPTY_STR; 4]],
                |mut acc, (idx, ((&go, rs), ts))| {
                    let min = *[go, rs, ts].iter().filter(|t| !t.is_zero()).min().unwrap();

                    acc[0][idx] = from_dur(&go, go == min);
                    acc[1][idx] = from_dur(&rs, rs == min);
                    acc[2][idx] = from_dur(&ts, ts == min);

                    if idx == 3 {
                        diffs[0][idx] = format!("<td><b>{}</b></td>", percent_diff(go, min));
                        diffs[1][idx] = format!("<td><b>{}</b></td>", percent_diff(rs, min));
                        diffs[2][idx] = format!("<td><b>{}</b></td>", percent_diff(ts, min));
                    } else {
                        diffs[0][idx] = format!("<td>{}</td>", percent_diff(go, min));
                        diffs[1][idx] = format!("<td>{}</td>", percent_diff(rs, min));
                        diffs[2][idx] = format!("<td>{}</td>", percent_diff(ts, min));
                    }

                    acc
                },
            );

            go_table.push_str(&res_as_row(day, &results[0]));
            go_table.push('\n');
            rust_table.push_str(&res_as_row(day, &results[1]));
            rust_table.push('\n');
            deno_table.push_str(&res_as_row(day, &results[2]));
            deno_table.push('\n');

            diff_table.push_str(&format!(
                "<tr>\n<td>{}</td>\n{}</tr>",
                day,
                diffs.iter().flatten().fold(String::new(), |mut acc, diff| {
                    acc.push_str(&diff);
                    acc.push('\n');
                    acc
                })
            ));
            diff_table.push('\n');
        });

    println!("Go");
    println!("{}", go_table);
    println!("Rust");
    println!("{}", rust_table);
    println!("TypeScript");
    println!("{}", deno_table);
    println!("Diffs");
    println!("{}", diff_table);
}

fn from_dur(dur: &Duration, lowest: bool) -> String {
    if dur.is_zero() {
        return "-".to_owned();
    }
    let d = if lowest {
        format!("**{:#?}**", dur)
    } else {
        format!("{:#?}", dur)
    };
    d.replace("µ", "&mu;")
}

fn res_as_row(day: usize, results: &[String; 4]) -> String {
    format!(
        "| {: <6}| {: <18}| {: <18}| {: <18}| {: <18}|",
        &day, &results[0], &results[1], &results[2], &results[3]
    )
}

fn percent_diff(dur: Duration, min: Duration) -> String {
    if dur.is_zero() {
        "-".to_owned()
    } else if dur == min {
        "0%".to_owned()
    } else {
        format!(
            "+{}%",
            (((((dur.as_nanos() as f64 / min.as_nanos() as f64) - 1f64) * 100f64).round()
                as usize)
                .separate_with_commas())
        )
    }
}

fn bench_rust(day: usize) -> [[Duration; 4]; 25] {
    let mut cmd = Command::new("cargo");
    let output = if day > 0 {
        cmd.args(["bench", "--package", &format!("day_{}", day)])
    } else {
        cmd.args(["bench", "--package", "day_*"])
    }
    .stdout(Stdio::piped())
    .spawn()
    .unwrap()
    .stdout
    .unwrap();

    let reader = BufReader::new(output);

    let mut results = [[Duration::default(); 4]; 25];
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| {
            println!("{}", line);
            if !line.contains("::bench") {
                return;
            }
            Regex::new(r".*?day_(\d+)::bench_([_\w]+).*bench:\s+([\d,]+)")
                .unwrap()
                .captures_iter(&line)
                .for_each(|c| {
                    let day = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    let part = match c.get(2).unwrap().as_str() {
                        "read_data" => 0,
                        "part_1" => 1,
                        "part_2" => 2,
                        _ => return,
                    };
                    let time = Duration::from_nanos(
                        c.get(3)
                            .unwrap()
                            .as_str()
                            .chars()
                            .filter_map(|c| {
                                if c.is_numeric() {
                                    Some(c as u8 - b'0')
                                } else {
                                    None
                                }
                            })
                            .fold(0u64, |acc, n| (acc * 10) + n as u64),
                    );

                    results[day - 1][part] = time;
                });
        });

    results
        .iter_mut()
        .for_each(|day| day[3] = day[..3].iter().sum());
    results
}

fn bench_go(day: usize) -> [[Duration; 4]; 25] {
    let mut cmd = Command::new("go");
    let output = if day > 0 {
        cmd.args(["test", "--bench=.", &format!("./go/day_{}/...", day)])
    } else {
        cmd.args(["test", "--bench=.", "./go/..."])
    }
    .stdout(Stdio::piped())
    .spawn()
    .unwrap()
    .stdout
    .unwrap();

    let reader = BufReader::new(output);

    let mut output = String::new();

    let mut results = [[Duration::default(); 4]; 25];
    let mut day = 0usize;
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| {
            println!("{}", line);
            if line.starts_with("pkg:") {
                day = Regex::new(r".*/day_(\d+)")
                    .unwrap()
                    .captures(&line)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap();
                return;
            }
            if !line.starts_with("Benchmark") {
                return;
            }
            Regex::new(r"Benchmark(\w+)-\d+\s+\d+\s+([\d\.]+) ns/op")
                .unwrap()
                .captures_iter(&line)
                .for_each(|c| {
                    let part = match c.get(1).unwrap().as_str() {
                        "ReadData" => 0,
                        "Part1" => 1,
                        "Part2" => 2,
                        _ => return,
                    };
                    let time = Duration::from_nanos(
                        c.get(2).unwrap().as_str().parse::<f64>().unwrap().round() as u64,
                    );

                    results[day - 1][part] = time;
                });

            output.push_str(&line);
            output.push('\n');
        });

    results
        .iter_mut()
        .for_each(|day| day[3] = day[..3].iter().sum());
    results
}

fn bench_deno(day: usize) -> [[Duration; 4]; 25] {
    let mut cmd = Command::new("deno");
    let output = if day > 0 {
        cmd.args([
            "test",
            "--allow-read",
            &format!("ts/day_{}", day),
            "--",
            "--bench",
        ])
    } else {
        cmd.args(["test", "--allow-read", "ts", "--", "--bench"])
    }
    .stdout(Stdio::piped())
    .spawn()
    .unwrap()
    .stdout
    .unwrap();

    let reader = BufReader::new(output);

    let mut results = [[Duration::default(); 4]; 25];
    let mut part = 0usize;
    let mut times = [Duration::default(); 4];
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| {
            println!("{}", line);
            if !line.ends_with("benchmarks ...") && line.starts_with("running ") {
                let day = Regex::new(r"running \d tests.*/day_(\d+)/main.test.ts")
                    .unwrap()
                    .captures(&line)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap()
                    - 1;
                times[3] = times[..3].iter().sum();
                results[day] = times;
                times = [Duration::default(); 4];
                return;
            }

            if line.starts_with("benchmark ") {
                part = match Regex::new(r"benchmark ([\w\d ]+).*")
                    .unwrap()
                    .captures(&line)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                {
                    "read data " => 0,
                    "part 1 " => 1,
                    "part 2 " => 2,
                    _ => return,
                };
                return;
            }
            if !line.contains("runs avg:") {
                return;
            }

            times[part] = Duration::from_nanos(
                (Regex::new(r".*: ([\d\.]+)ms")
                    .unwrap()
                    .captures(&line)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<f64>()
                    .unwrap()
                    * 1_000_000.0)
                    .round() as u64,
            );
        });

    results
}
