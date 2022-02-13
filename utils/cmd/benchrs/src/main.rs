#[deny(clippy::all)]
use regex::Regex;
use std::alloc::{GlobalAlloc, Layout, System};
use std::env;
use std::fmt::{Debug, Display, Write};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use std::time::{Duration, Instant};

macro_rules! run_day {
    ($module:ident) => {
        println!("Running benchmarks for {}...", stringify!($module));
        let mut result_file =
            ResultFile::new(format!("./bench/results/rs/{}.csv", stringify!($module)).as_str())
                .unwrap();

        let results = run_bench(Parts::ReadData, || {
            let data = $module::read_data("./data");
            assert_ne!(data, $module::Input::default());
        });

        result_file.write(&results).unwrap();

        println!("Parse:\n\t{}\n", process_results(results));

        let data = $module::read_data("./data");

        let results = run_bench(Parts::One, || {
            assert_eq!($module::part_1(&data), $module::PART_1);
        });

        result_file.write(&results).unwrap();

        println!("Part 1:\n\t{}\n", process_results(results));

        let results = run_bench(Parts::Two, || {
            assert_eq!($module::part_2(&data), $module::PART_2);
        });

        result_file.write(&results).unwrap();

        println!("Part 2:\n\t{}\n", process_results(results));

        let results = run_bench(Parts::Total, || {
            let data = $module::read_data("./data");
            assert_ne!(data, $module::Input::default());
            assert_eq!($module::part_1(&data), $module::PART_1);
            assert_eq!($module::part_2(&data), $module::PART_2);
        });

        result_file.write(&results).unwrap();

        println!("Total:\n\t{}\n", process_results(results));

        // Perform a benchmark run using the built-in Cargo benchmark
        // to provide a baseline for the impact of the custom counting
        // memory allocator
        // Writes the shown median, plus derived min. and max. times for each part
        // to an output CSV.
        println!("Running 'cargo bench {}'...", stringify!($module));
        cargo_bench(stringify!($module));
    };
}

pub(crate) enum Parts {
    ReadData,
    One,
    Two,
    Total,
}

impl Display for Parts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Parts::ReadData => f.write_str("read"),
            Parts::One => f.write_str("part 1"),
            Parts::Two => f.write_str("part 2"),
            Parts::Total => f.write_str("total"),
        }
    }
}

fn main() {
    if env::args().len() > 1 {
        match env::args().nth(1).unwrap().parse::<usize>().unwrap() {
            1 => {
                run_day!(day_1);
            }
            2 => {
                run_day!(day_2);
            }
            3 => {
                run_day!(day_3);
            }
            4 => {
                run_day!(day_4);
            }
            5 => {
                run_day!(day_5);
            }
            6 => {
                run_day!(day_6);
            }
            7 => {
                run_day!(day_7);
            }
            8 => {
                run_day!(day_8);
            }
            9 => {
                run_day!(day_9);
            }
            10 => {
                run_day!(day_10);
            }
            11 => {
                run_day!(day_11);
            }
            12 => {
                run_day!(day_12);
            }
            13 => {
                run_day!(day_13);
            }
            14 => {
                run_day!(day_14);
            }
            15 => {
                run_day!(day_15);
            }
            16 => {
                run_day!(day_16);
            }
            17 => {
                run_day!(day_17);
            }
            // 18 => {run_day!(day_18);},
            19 => {
                run_day!(day_19);
            }
            20 => {
                run_day!(day_20);
            }
            21 => {
                run_day!(day_21);
            }
            // 22 => {run_day!(day_22);},
            // 23 => {run_day!(day_23);},
            // 24 => {run_day!(day_24);},
            25 => {
                run_day!(day_25);
            }
            _ => {}
        }
        return;
    };

    run_day!(day_1);
    run_day!(day_2);
    run_day!(day_3);
    run_day!(day_4);
    run_day!(day_5);
    run_day!(day_6);
    run_day!(day_7);
    run_day!(day_8);
    run_day!(day_9);
    run_day!(day_10);
    run_day!(day_11);
    run_day!(day_12);
    run_day!(day_13);
    run_day!(day_14);
    run_day!(day_15);
    run_day!(day_16);
    run_day!(day_17);
    // run_day!(day_18);
    run_day!(day_19);
    run_day!(day_20);
    run_day!(day_21);
    // run_day!(day_22);
    // run_day!(day_23);
    // run_day!(day_24);
    run_day!(day_25);
}

pub struct Counter;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static MAX_ALLOC: AtomicUsize = AtomicUsize::new(0);
static NUM_ALLOC: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counter {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), SeqCst);
            MAX_ALLOC.fetch_max(layout.size(), SeqCst);
            NUM_ALLOC.fetch_add(1, SeqCst);
        }
        ret
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), SeqCst);
    }
}

impl Counter {
    fn reset() {
        MAX_ALLOC.store(0, SeqCst);
        NUM_ALLOC.store(0, SeqCst);
    }
}

#[global_allocator]
pub static mut A: Counter = Counter;

struct BenchResult {
    results: Vec<(u64, usize, usize)>,
    part: Parts,
}

impl BenchResult {
    fn new(part: Parts) -> Self {
        Self {
            results: Vec::with_capacity(1024),
            part,
        }
    }

    fn add(&mut self, elapsed: u64, max_alloc: usize, num_alloc: usize) {
        self.results.push((elapsed, max_alloc, num_alloc));
    }

    fn means(&self) -> (Duration, Bytes, usize) {
        let means = self
            .results
            .iter()
            .fold((0u64, 0usize, 0usize), |acc, result| {
                (acc.0 + result.0, acc.1 + result.1, acc.2 + result.2)
            });
        (
            Duration::from_nanos(means.0 / self.results.len() as u64),
            Bytes(means.1 / self.results.len()),
            means.2 / self.results.len(),
        )
    }

    fn to_csv(&self) -> String {
        let mut out = String::with_capacity(128 * self.results.len());

        for (run, result) in self.results.iter().enumerate().take(self.results.len()) {
            out.write_fmt(format_args!(
                "{},{},{},{},{}\n",
                self.part,
                run + 1,
                result.0,
                result.1,
                result.2
            ))
            .unwrap();
        }

        out
    }
}

struct ResultFile {
    file: std::fs::File,
}

impl ResultFile {
    fn new(filepath: &str) -> std::io::Result<Self> {
        let mut file = std::fs::File::options()
            .truncate(true)
            .write(true)
            .create(true)
            .open(filepath)?;

        std::io::Write::write_all(
            &mut file,
            b"Part,Run,Elapsed (ns),Max. memory (B), Num. allocations\n",
        )?;

        Ok(Self { file })
    }

    fn write(&mut self, results: &BenchResult) -> std::io::Result<()> {
        std::io::Write::write_all(&mut self.file, results.to_csv().as_bytes())
    }
}

const RUNTIME_LIMIT: Duration = Duration::from_secs(30);
const RUNTIME_TARGET: Duration = Duration::from_millis(1);
const RUN_LIMIT: usize = 10240;
const MIN_RUNS: usize = 100;

fn run_bench<F: Fn()>(part: Parts, closure_to_time: F) -> BenchResult {
    let mut out = BenchResult::new(part);

    Counter::reset();
    let mut total_time = Duration::default();
    let mut total_runs = 0;

    let mut n = 1;
    while (total_runs < MIN_RUNS) || (total_time < RUNTIME_LIMIT && total_runs < RUN_LIMIT) {
        total_runs += 1;
        let mut start: Instant;
        let mut elapsed: Duration;
        let mut baseline: usize;
        let mut num_alloc: usize;
        let mut max_alloc: usize;
        loop {
            Counter::reset();
            baseline = ALLOCATED.load(SeqCst);
            start = Instant::now();
            for _ in 0..n {
                closure_to_time();
            }
            elapsed = start.elapsed();
            num_alloc = NUM_ALLOC.load(SeqCst);
            max_alloc = MAX_ALLOC.load(SeqCst);
            if elapsed >= RUNTIME_TARGET {
                break;
            }
            n *= (RUNTIME_TARGET.as_nanos() / elapsed.as_nanos()).max(2) as usize;
        }

        total_time += elapsed;
        out.add(
            // Counter alloc is 11ns slower per allocation than the system
            // allocator, so adjust the results to remove this additional latency
            // to avoid unfairly penalising functions that allocate lots of memory
            elapsed.as_nanos() as u64 / n as u64,
            if max_alloc > baseline {
                max_alloc - baseline
            } else {
                max_alloc
            },
            num_alloc / n,
        );
    }
    Counter::reset();

    out
}

fn process_results(results: BenchResult) -> String {
    let (time, max_mem, num_mem) = results.means();

    format!(
        "Took: {:#?}\tMax memory: {:?}\t# allocations: {}",
        time, max_mem, num_mem,
    )
}

struct Bytes(usize);

impl Debug for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (shift, suffix) = match self.0.leading_zeros() {
            0..=3 => (60, "EB"),
            4..=13 => (50, "PB"),
            14..=23 => (40, "TB"),
            24..=33 => (30, "GB"),
            34..=43 => (20, "MB"),
            44..=53 => (10, "KB"),
            _ => {
                return f.write_fmt(format_args!("{} B", self.0));
            }
        };

        let byte_count = self.0 >> shift;
        let remainder = ((((self.0 & !(byte_count << shift)) >> (shift - 10)) as f64 / 1024.0)
            * 1000.0)
            .trunc();
        if remainder > 0.0 {
            f.write_fmt(format_args!("{}.{} {}", byte_count, remainder, suffix))
        } else {
            f.write_fmt(format_args!("{} {}", byte_count, suffix))
        }
    }
}

fn cargo_bench(day: &str) {
    let mut cargo_result_file =
        ResultFile::new(format!("./bench/results/rs/{}_cargo.csv", day).as_str()).unwrap();

    let mut cmd = Command::new("cargo");
    let bench = cmd
        .args(["bench", "-p", day])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout_reader = BufReader::new(bench.stdout.unwrap());

    stdout_reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| {
            println!("{}", line);
            if !line.contains(" bench: ") {
                return;
            }
            Regex::new(r".*?tests::([_\w]+):.*bench:\s+([\d,]+).*\(\+/- ([\d,]+)")
                .unwrap()
                .captures_iter(&line)
                .for_each(|c| {
                    let part = match c.get(1).unwrap().as_str() {
                        "read_data" => Parts::ReadData,
                        "part_1" => Parts::One,
                        "part_2" => Parts::Two,
                        "total" => Parts::Total,
                        _ => return,
                    };
                    let median = Duration::from_nanos(
                        c.get(2)
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
                    let width = Duration::from_nanos(
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

                    std::io::Write::write_all(
                        &mut cargo_result_file.file,
                        format!("{},{},{},{},{}\n", part, 1, median.as_nanos(), 0, 0,).as_bytes(),
                    )
                    .unwrap();
                    std::io::Write::write_all(
                        &mut cargo_result_file.file,
                        format!(
                            "{},{},{},{},{}\n",
                            part,
                            2,
                            (median - (width / 2)).as_nanos(),
                            0,
                            0,
                        )
                        .as_bytes(),
                    )
                    .unwrap();
                    std::io::Write::write_all(
                        &mut cargo_result_file.file,
                        format!(
                            "{},{},{},{},{}\n",
                            part,
                            3,
                            (median + ((width + Duration::from_nanos(1)) / 2)).as_nanos(),
                            0,
                            0,
                        )
                        .as_bytes(),
                    )
                    .unwrap();
                });
        });
}
