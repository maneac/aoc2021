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

A utility to generate the below results can be found in `utils/benchmarks`.

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
    <td>+52%</td>
    <td>0%</td>
    <td><b>+2%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>+150%</td>
    <td><b>0%</b></td>
    <td>+324%</td>
    <td>+4,482%</td>
    <td>+4,591%</td>
    <td><b>+589%</b></td>
  </tr>
  <tr>
    <td>2</td>
    <td>0%</td>
    <td>+46,738%</td>
    <td>+59,403%</td>
    <td><b>+3,450%</b></td>
    <td>+68%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>+1,389%</td>
    <td>+2,847%</td>
    <td>+1,338%</td>
    <td><b>+877%</b></td>
  </tr>
  <tr>
    <td>3</td>
    <td>+174%</td>
    <td>+181%</td>
    <td>0%</td>
    <td><b>+20%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>+678,592%</td>
    <td><b>0%</b></td>
    <td>+259%</td>
    <td>+2,698%</td>
    <td>+203,233%</td>
    <td><b>+180%</b></td>
  </tr>
  <tr>
    <td>4</td>
    <td>+1,876%</td>
    <td>+37%</td>
    <td>0%</td>
    <td><b>+48%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>+54%</td>
    <td><b>0%</b></td>
    <td>+1,450%</td>
    <td>+2,870%</td>
    <td>+951%</td>
    <td><b>+1,044%</b></td>
  </tr>
  <tr>
    <td>5</td>
    <td>+1,779%</td>
    <td>+693%</td>
    <td>+878%</td>
    <td><b>+802%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>+1,256%</td>
    <td>+5,413%</td>
    <td>+3,338%</td>
    <td><b>+4,259%</b></td>
  </tr>
  <tr>
    <td>6</td>
    <td>+57%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>+35%</b></td>
    <td>0%</td>
    <td>+12%</td>
    <td>+14%</td>
    <td><b>0%</b></td>
    <td>+421%</td>
    <td>+781%</td>
    <td>+265%</td>
    <td><b>+392%</b></td>
  </tr>
  <tr>
    <td>7</td>
    <td>+188%</td>
    <td>+93%</td>
    <td>+79%</td>
    <td><b>+82%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>+813%</td>
    <td>+2,891%</td>
    <td>+364%</td>
    <td><b>+376%</b></td>
  </tr>
  <tr>
    <td>8</td>
    <td>+161%</td>
    <td>+14%</td>
    <td>+896%</td>
    <td><b>+340%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>+395%</td>
    <td>+1,050%</td>
    <td>+5,349%</td>
    <td><b>+1,627%</b></td>
  </tr>
  <tr>
    <td>9</td>
    <td>+1,958%</td>
    <td>+9,679%</td>
    <td>+387%</td>
    <td><b>+470%</b></td>
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
    <td>+446%</td>
    <td>+188%</td>
    <td><b>+120%</b></td>
    <td>+1,036%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>+191%</td>
    <td>+1,555%</td>
    <td>+356%</td>
    <td><b>+408%</b></td>
  </tr>
  <tr>
    <td>11</td>
    <td>+351%</td>
    <td>+744%</td>
    <td>+465%</td>
    <td><b>+512%</b></td>
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
    <td>12</td>
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

| Day   | Parse             | Part 1            | Part 2            | Total             |
|:-----:|:-----------------:|:-----------------:|:-----------------:|:-----------------:|
| 1     | 26.087&mu;s       | 636ns             | **1.535&mu;s**    | 28.258&mu;s       |
| 2     | **10.964&mu;s**   | 343.321&mu;s      | 347.495&mu;s      | 701.78&mu;s       |
| 3     | 155.021&mu;s      | 17.627&mu;s       | **12ns**          | 172.66&mu;s       |
| 4     | 405.88&mu;s       | 143.912&mu;s      | **281.213&mu;s**  | 831.005&mu;s      |
| 5     | 463.731&mu;s      | 10.464938ms       | 15.407499ms       | 26.336168ms       |
| 6     | 4.827&mu;s        | **318ns**         | **986ns**         | 6.131&mu;s        |
| 7     | 51.782&mu;s       | 414ns             | 1.21804ms         | 1.270236ms        |
| 8     | 90.403&mu;s       | 756ns             | 115.152&mu;s      | 206.311&mu;s      |
| 9     | 313.781&mu;s      | 716.535&mu;s      | 5.284735ms        | 6.315051ms        |
| 10    | **4.542&mu;s**    | 130.018&mu;s      | 133.546&mu;s      | 268.106&mu;s      |
| 11    | 14.943&mu;s       | 131.827&mu;s      | 366.356&mu;s      | 513.126&mu;s      |
| 12    | -                 | -                 | -                 | -                 |

### Rust

| Day   | Parse             | Part 1            | Part 2            | Total             |
|:-----:|:-----------------:|:-----------------:|:-----------------:|:-----------------:|
| 1     | **23.371&mu;s**   | **419ns**         | 3.836&mu;s        | **27.626&mu;s**   |
| 2     | 18.453&mu;s       | **733ns**         | **584ns**         | **19.77&mu;s**    |
| 3     | **56.649&mu;s**   | **6.275&mu;s**    | 81.443&mu;s       | **144.367&mu;s**  |
| 4     | **20.543&mu;s**   | **105.424&mu;s**  | 433.88&mu;s       | **559.847&mu;s**  |
| 5     | **24.686&mu;s**   | **1.320214ms**    | **1.5756ms**      | **2.9205ms**      |
| 6     | **3.07&mu;s**     | 357ns             | 1.126&mu;s        | **4.553&mu;s**    |
| 7     | **17.999&mu;s**   | **214ns**         | **679.646&mu;s**  | **697.859&mu;s**  |
| 8     | **34.646&mu;s**   | **661ns**         | **11.561&mu;s**   | **46.868&mu;s**   |
| 9     | **15.247&mu;s**   | **7.327&mu;s**    | **1.084958ms**    | **1.107532ms**    |
| 10    | 51.584&mu;s       | **23.803&mu;s**   | **46.302&mu;s**   | **121.689&mu;s**  |
| 11    | **3.311&mu;s**    | **15.625&mu;s**   | **64.875&mu;s**   | **83.811&mu;s**   |
| 12    | **7.092&mu;s**    | **60.067&mu;s**   | **1.922788ms**    | **1.989947ms**    |

### TypeScript (Deno)

| Day   | Parse             | Part 1            | Part 2            | Total             |
|:-----:|:-----------------:|:-----------------:|:-----------------:|:-----------------:|
| 1     | 99.2&mu;s         | 19.2&mu;s         | 72&mu;s           | 190.4&mu;s        |
| 2     | 163.2&mu;s        | 21.6&mu;s         | 8.4&mu;s          | 193.2&mu;s        |
| 3     | 203.6&mu;s        | 175.6&mu;s        | 24.4&mu;s         | 403.6&mu;s        |
| 4     | 318.4&mu;s        | 3.1316ms          | 2.9568ms          | 6.4068ms          |
| 5     | 334.8&mu;s        | 72.7892ms         | 54.1704ms         | 127.2944ms        |
| 6     | 16&mu;s           | 2.8&mu;s          | 3.6&mu;s          | 22.4&mu;s         |
| 7     | 164.4&mu;s        | 6.4&mu;s          | 3.154ms           | 3.3248ms          |
| 8     | 171.6&mu;s        | 7.6&mu;s          | 630&mu;s          | 809.2&mu;s        |
| 9     | -                 | -                 | -                 | -                 |
| 10    | 13.2&mu;s         | 394&mu;s          | 211.2&mu;s        | 618.4&mu;s        |
| 11    | -                 | -                 | -                 | -                 |
| 12    | -                 | -                 | -                 | -                 |

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
