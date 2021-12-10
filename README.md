# Advent of Code 2021 Solutions

[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](http://unlicense.org/)
[![Rust](https://github.com/maneac/aoc2021/actions/workflows/rust.yml/badge.svg)](https://github.com/maneac/aoc2021/actions/workflows/rust.yml)
[![Go](https://github.com/maneac/aoc2021/actions/workflows/golang.yml/badge.svg)](https://github.com/maneac/aoc2021/actions/workflows/golang.yml)
[![Deno](https://github.com/maneac/aoc2021/actions/workflows/deno.yml/badge.svg)](https://github.com/maneac/aoc2021/actions/workflows/deno.yml)

Solutions for 2021's [Advent of Code](https://adventofcode.com/2021). Instructions for running each language can be found in its folder.

## Populating A New Day

Ensure `AOC_SESSION_TOKEN` (sent as a header in requests on the Advent of Code website) is set.

### Part 1 + Data

```bash
# Using the stable Rust toolchain
cargo run --release
```

### Part 2

```bash
# Using the stable Rust toolchain
cargo run --release -- --part-2
```

## Solution Benchmark Results

The commands used and specifics of each benchmark can be found in each langauge's folder and source.

Reminder: s = 1000ms, ms = 1000&mu;s, &mu;s = 1000ns

**Key**:

- **bold** - fastest language for that day / part.
- *italics* - different implementation, not necessarily comparable.

\* Due to framework limitations, all TypeScript benchmark results are measured to the nearest &mu;s.

### Relative Time Taken

Percentage increase compared to the fastest for each day's part.

<table>
  <tr>
    <th rowspan=2>Day</th>
    <th colspan=4>Go</th>
    <th colspan=4>Rust</th>
    <th colspan=4>TypeScript</th>
  </tr>
  <tr>
    <th>Parse</th>
    <th>Part 1</th>
    <th>Part 2</th>
    <th>Total</th>
    <th>Parse</th>
    <th>Part 1</th>
    <th>Part 2</th>
    <th>Total</th>
    <th>Parse</th>
    <th>Part 1</th>
    <th>Part 2</th>
    <th>Total</th>
  </tr>
  <tr>
    <td>1</td>
    <td>+13%</td>
    <td>+56%</td>
    <td>0%</td>
    <td><b>+4%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>+147%</td>
    <td><b>0%</b></td>
    <td>+405%</td>
    <td>+8,659%</td>
    <td>+5,566%</td>
    <td><b>+782%</b></td>
  </tr>
  <tr>
    <td>2</td>
    <td>0%</td>
    <td>+46,200%</td>
    <td>+57,887%</td>
    <td><b>+3,557%</b></td>
    <td>+61%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>+1,544%</td>
    <td>+4,260%</td>
    <td>+2,616%</td>
    <td><b>+1,104%</b></td>
  </tr>
  <tr>
    <td>3</td>
    <td><em>+176%</em></td>
    <td><em>+190%</em></td>
    <td><em>0%</em></td>
    <td><em><b>+21%</b></em></td>
    <td>0%</td>
    <td>0%</td>
    <td>+671,327%</td>
    <td><b>0%</b></td>
    <td>+223%</td>
    <td>+3,451%</td>
    <td>+433,957%</td>
    <td><b>+246%</b></td>
  </tr>
  <tr>
    <td>4</td>
    <td>+1,885%</td>
    <td>+38%</td>
    <td>0%</td>
    <td><b>+51%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>+49%</td>
    <td><b>0%</b></td>
    <td>+1,474%</td>
    <td>+2,902%</td>
    <td>+924%</td>
    <td><b>+1,056%</b></td>
  </tr>
  <tr>
    <td>5</td>
    <td>+1,733%</td>
    <td>+733%</td>
    <td>+911%</td>
    <td><b>+839%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>+1,427%</td>
    <td>+5,592%</td>
    <td>+3,437%</td>
    <td><b>+4,383%</b></td>
  </tr>
  <tr>
    <td>6</td>
    <td>+54%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>+33%</b></td>
    <td>0%</td>
    <td>+14%</td>
    <td>+13%</td>
    <td><b>0%</b></td>
    <td>+412%</td>
    <td>+277%</td>
    <td>+385%</td>
    <td><b>+377%</b></td>
  </tr>
  <tr>
    <td>7</td>
    <td>+182%</td>
    <td>+93%</td>
    <td>+74%</td>
    <td><b>+77%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>+840%</td>
    <td>+1,778%</td>
    <td>+260%</td>
    <td><b>+275%</b></td>
  </tr>
  <tr>
    <td>8</td>
    <td>+166%</td>
    <td>+16%</td>
    <td>+1,034%</td>
    <td><b>+364%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td><b>-</b></td>
  </tr>
  <tr>
    <td>9</td>
    <td>+2168%</td>
    <td>+9520%</td>
    <td>+404%</td>
    <td><b>+487%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td><b>-</b></td>
  </tr>
  <tr>
    <td>10</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td><b>-</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td><b>-</b></td>
  </tr>
</table>

### Go

| Day   | Parse             | Part 1        | Part 2            | Total             |
|:-----:|:-----------------:|:-------------:|:-----------------:|:-----------------:|
| 1     | 26.021&mu;s       | 640.2ns       | **1.553&mu;s**    | 28.214&mu;s       |
| 2     | **10.943&mu;s**   | 339.841&mu;s  | 341.544&mu;s      | 692.328&mu;s      |
| 3     | *154.031&mu;s*    | *17.639&mu;s* | ***11.98ns***     | *171.682&mu;s*    |
| 4     | 403.665&mu;s      | 144.429&mu;s  | **286.581&mu;s**  | 834.675&mu;s      |
| 5     | 461.056&mu;s      | 10.627ms      | 15.657ms          | 26.746ms          |
| 6     | 4.828&mu;s        | **318.3ns**   | **990.3ns**       | 6.137&mu;s        |
| 7     | 51.669&mu;s       | 411.6ns       | 1.221ms           | 1.274ms           |
| 8     | 90.356&mu;s       | 763.2ns       | 118.084&mu;s      | 209.203&mu;s      |
| 9     | 311.895&mu;s      | 708.992&mu;s  | 5.433ms           | 6.453ms           |
| 10    | -                 | -             | -                 | -                 |

### Rust

| Day   | Parse             | Part 1            | Part 2            | Total             |
|:-----:|:-----------------:|:-----------------:|:-----------------:|:-----------------:|
| 1     | **22.965&mu;s**   | **411ns**         | 3.833&mu;s        | **27.209&mu;s**   |
| 2     | 17.61&mu;s        | **734ns**         | **589ns**         | **18.933&mu;s**   |
| 3     | **55.775&mu;s**   | **6.083&mu;s**    | 80.437&mu;s       | **142.295&mu;s**  |
| 4     | **20.331&mu;s**   | **104.342&mu;s**  | 428.096&mu;s      | **552.769&mu;s**  |
| 5     | **25.155&mu;s**   | **1.275ms**       | **1.548ms**       | **2.848ms**       |
| 6     | **3.128&mu;s**    | 364ns             | 1.123&mu;s        | **4.615&mu;s**    |
| 7     | **18.305&mu;s**   | **213ns**         | **700.813&mu;s**  | **719.331&mu;s**  |
| 8     | **34.029&mu;s**   | **659ns**         | **10.41&mu;s**    | **45.098&mu;s**   |
| 9     | **13.753&mu;s**   | **7.37&mu;s**     | 1.078ms           | **1.099ms**       |
| 10    | **51.669&mu;s**   | **23.672&mu;s**   | **46.297&mu;s**   | **121.638&mu;s**  |

### TypeScript (Deno)

| Day   | Parse     | Part 1    | Part 2    | Total     |
|:-----:|:---------:|:---------:|:---------:|:---------:|
| 1     | 116&mu;s  | 36&mu;s   | 88&mu;s   | 240&mu;s  |
| 2     | 180&mu;s  | 32&mu;s   | 16&mu;s   | 228&mu;s  |
| 3     | 224&mu;s  | 216&mu;s  | 52&mu;s   | 492&mu;s  |
| 4     | 320&mu;s  | 3.132ms   | 2.936ms   | 6.388ms   |
| 5     | 348&mu;s  | 72.576ms  | 54.76ms   | 127.684ms |
| 6     | 16&mu;s   | 1.2&mu;s  | 4.8&mu;s  | 22&mu;s   |
| 7     | 172&mu;s  | 4&mu;s    | 2.524ms   | 2.7ms     |
| 8     | -         | -         | -         | -         |
| 9     | -         | -         | -         | -         |
| 10    | -         | -         | -         | -         |

### System Information

```sh
> go version
go version go1.17.4 linux/amd64

> rustc --version
rustc 1.59.0-nightly (e6b883c74 2021-12-08)

> deno --version
deno 1.16.3 (release, x86_64-unknown-linux-gnu)
v8 9.7.106.5
typescript 4.4.2
```

All benchmarks were conducted on an Intel Core i7 12700k, with 32GB RAM and an NVMe SSD, using OpenSUSE Tumbleweed 20211207-0 via Windows 11's WSL.
