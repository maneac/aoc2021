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
cargo run --release
```

### Part 2

```bash
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
    <td>+12%</td>
    <td>+55%</td>
    <td>0%</td>
    <td><b>+3%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>+141%</td>
    <td><b>0%</b></td>
    <td>+401%</td>
    <td>+8,638%</td>
    <td>+5,566%</td>
    <td><b>+779%</b></td>
  </tr>
  <tr>
    <td>2</td>
    <td>0%</td>
    <td>+47,035%</td>
    <td>+58,991%</td>
    <td><b>+3,455%</b></td>
    <td>+66%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>+1,544%</td>
    <td>+4,338%</td>
    <td>+2,668%</td>
    <td><b>+1,071%</b></td>
  </tr>
  <tr>
    <td>3</td>
    <td><em>+160%</em></td>
    <td><em>+190%</em></td>
    <td><em>0%</em></td>
    <td><em><b>+19%</b></em></td>
    <td>0%</td>
    <td>0%</td>
    <td>+656,327%</td>
    <td><b>0%</b></td>
    <td>+279%</td>
    <td>+3,446%</td>
    <td>+433,957%</td>
    <td><b>+242%</b></td>
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
    <td>+464%</td>
    <td>+2,935</td>
    <td>+6,624%</td>
    <td><b>+1,922</b></td>
  </tr>
  <tr>
    <td>9</td>
    <td>+2,168%</td>
    <td>+9,520%</td>
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
    <td>0%</td>
    <td>+449%</td>
    <td>+186%</td>
    <td><b>+119%</b></td>
    <td>+1,063%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>+170%</td>
    <td>+1,556%</td>
    <td>+384%</td>
    <td><b>+416%</b></td>
  </tr>
    <tr>
    <td>11</td>
    <td>+417%</td>
    <td>+726%</td>
    <td>+452%</td>
    <td><b>+500%</b></td>
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

| Day   | Parse             | Part 1            | Part 2            | Total             |
|:-----:|:-----------------:|:-----------------:|:-----------------:|:-----------------:|
| 1     | 26.021&mu;s       | 640.2ns           | **1.553&mu;s**    | 28.214&mu;s       |
| 2     | **10.943&mu;s**   | 339.841&mu;s      | 341.544&mu;s      | 692.328&mu;s      |
| 3     | *154.031&mu;s*    | *17.639&mu;s*     | ***11.98ns***     | *171.682&mu;s*    |
| 4     | 403.665&mu;s      | 144.429&mu;s      | **286.581&mu;s**  | 834.675&mu;s      |
| 5     | 461.056&mu;s      | 10.627ms          | 15.657ms          | 26.746ms          |
| 6     | 4.828&mu;s        | **318.3ns**       | **990.3ns**       | 6.137&mu;s        |
| 7     | 51.669&mu;s       | 411.6ns           | 1.221ms           | 1.274ms           |
| 8     | 90.356&mu;s       | 763.2ns           | 118.084&mu;s      | 209.203&mu;s      |
| 9     | 311.895&mu;s      | 708.992&mu;s      | 5.433ms           | 6.453ms           |
| 10    | **4.444&mu;s**    | 130.01&mu;s       | 132.407&mu;s      | 266.861&mu;s      |
| 11    | 14.884&mu;s       | 130.646&mu;s      | 362.554&mu;s      | 508.084&mu;s      |

### Rust

| Day   | Parse             | Part 1            | Part 2            | Total             |
|:-----:|:-----------------:|:-----------------:|:-----------------:|:-----------------:|
| 1     | **23.161&mu;s**   | **412ns**         | 3.746&mu;s        | **27.319&mu;s**   |
| 2     | 18.176&mu;s       | **721ns**         | **578ns**         | **19.475&mu;s**   |
| 3     | **59.172&mu;s**   | **6.091&mu;s**    | 78.64&mu;s        | **143.903&mu;s**  |
| 4     | **20.397&mu;s**   | **103.839&mu;s**  | 427.811&mu;s      | **552.047&mu;s**  |
| 5     | **24.159&mu;s**   | **1.212ms**       | **1.436ms**       | **1.582ms**       |
| 6     | **3.02&mu;s**     | 355ns             | 1.121&mu;s        | **4.496&mu;s**    |
| 7     | **17.987&mu;s**   | **209ns**         | **681.347&mu;s**  | **699.543&mu;s**  |
| 8     | **33.692&mu;s**   | **658ns**         | **10.155&mu;s**   | **44.505&mu;s**   |
| 9     | **14.784&mu;s**   | **7.227&mu;s**    | **1.080ms**       | **1.102ms**       |
| 10    | 51.618&mu;s       | **23.769&mu;s**   | **46.221&mu;s**   | **121.608&mu;s**  |
| 11    | **3.324&mu;s**    | **15.487&mu;s**   | **66.171&mu;s**   | **84.982&mu;s**   |

### TypeScript (Deno)

| Day   | Parse             | Part 1            | Part 2            | Total             |
|:-----:|:-----------------:|:-----------------:|:-----------------:|:-----------------:|
| 1     | 116&mu;s          | 36&mu;s           | 88&mu;s           | 240&mu;s          |
| 2     | 180&mu;s          | 32&mu;s           | 16&mu;s           | 228&mu;s          |
| 3     | 224&mu;s          | 216&mu;s          | 52&mu;s           | 492&mu;s          |
| 4     | 320&mu;s          | 3.132ms           | 2.936ms           | 6.388ms           |
| 5     | 348&mu;s          | 72.576ms          | 54.76ms           | 127.684ms         |
| 6     | 16&mu;s           | 1.2&mu;s          | 4.8&mu;s          | 22&mu;s           |
| 7     | 172&mu;s          | 4&mu;s            | 2.524ms           | 2.7ms             |
| 8     | 192&mu;s          | 20&mu;s           | 700&mu;s          | 912&mu;s          |
| 9     | -                 | -                 | -                 | -                 |
| 10    | 12&mu;s           | 392&mu;s          | 224&mu;s          | 628&mu;s          |
| 11    | -                 | -                 | -                 | -                 |

### System Information

```sh
> go version
go version go1.17.4 linux/amd64

> rustc --version
rustc 1.59.0-nightly (928783de6 2021-12-11)

> deno --version
deno 1.16.3 (release, x86_64-unknown-linux-gnu)
v8 9.7.106.5
typescript 4.4.2
```

All benchmarks were conducted on an Intel Core i7 12700k, with 32GB RAM and an NVMe SSD, using OpenSUSE Tumbleweed 20211207-0 via Windows 11's WSL.
