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
    <td>+10%</td>
    <td>+52%</td>
    <td>0%</td>
    <td><b>+1%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>+149%</td>
    <td><b>0%</b></td>
    <td>+330%</td>
    <td>+3,910%</td>
    <td>+4,250%</td>
    <td><b>+565%</b></td>
  </tr>
  <tr>
    <td>2</td>
    <td>0%</td>
    <td>+46,577%</td>
    <td>+58,672%</td>
    <td><b>+3,497%</b></td>
    <td>+62%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>+1,364%</td>
    <td>+2,620%</td>
    <td>+1,788%</td>
    <td><b>+900%</b></td>
  </tr>
  <tr>
    <td>3</td>
    <td>+173%</td>
    <td>+191%</td>
    <td>0%</td>
    <td><b>+22%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>+636,992%</td>
    <td><b>0%</b></td>
    <td>+256%</td>
    <td>+2,829%</td>
    <td>+176,567%</td>
    <td><b>+185%</b></td>
  </tr>
  <tr>
    <td>4</td>
    <td>+1,890%</td>
    <td>+39%</td>
    <td>0%</td>
    <td><b>+49%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>+52%</td>
    <td><b>0%</b></td>
    <td>+1,405%</td>
    <td>+2,937%</td>
    <td>+943%</td>
    <td><b>+1,055%</b></td>
  </tr>
  <tr>
    <td>5</td>
    <td>+1,868%</td>
    <td>+779%</td>
    <td>+986%</td>
    <td><b>+900%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>+1,350%</td>
    <td>+6,107%</td>
    <td>+3,821%</td>
    <td><b>+4,829%</b></td>
  </tr>
  <tr>
    <td>6</td>
    <td>+55%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>+33%</b></td>
    <td>0%</td>
    <td>+14%</td>
    <td>+14%</td>
    <td><b>0%</b></td>
    <td>+415%</td>
    <td>+1,203%</td>
    <td>+232%</td>
    <td><b>+409%</b></td>
  </tr>
  <tr>
    <td>7</td>
    <td>+163%</td>
    <td>+96%</td>
    <td>+79%</td>
    <td><b>+81%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>+748%</td>
    <td>+2,579%</td>
    <td>+366%</td>
    <td><b>+377%</b></td>
  </tr>
  <tr>
    <td>8</td>
    <td>+164%</td>
    <td>+15%</td>
    <td>+1,023%</td>
    <td><b>+356%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>+405%</td>
    <td>+1,324%</td>
    <td>+6,222%</td>
    <td><b>+1,737%</b></td>
  </tr>
  <tr>
    <td>9</td>
    <td>+1,948%</td>
    <td>+9,639%</td>
    <td>+395%</td>
    <td><b>+478%</b></td>
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
    <td>+420%</td>
    <td>+183%</td>
    <td><b>+117%</b></td>
    <td>+1,029%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>+207%</td>
    <td>+1,480%</td>
    <td>+350%</td>
    <td><b>+401%</b></td>
  </tr>
  <tr>
    <td>11</td>
    <td>+340%</td>
    <td>+756%</td>
    <td>+467%</td>
    <td><b>+515%</b></td>
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
  <tr>
    <td>13</td>
    <td>+1,187%</td>
    <td>+314%</td>
    <td>+289%</td>
    <td><b>+582%</b></td>
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
    <td>14</td>
    <td>+23%</td>
    <td>+103%</td>
    <td>+110%</td>
    <td><b>+104%</b></td>
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
    <td>15</td>
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
| 1     | 25.180&mu;s       | 620.000ns         | **1.499&mu;s**    | 27.299&mu;s       |
| 2     | **10.793&mu;s**   | 329.539&mu;s      | 336.177&mu;s      | 676.509&mu;s      |
| 3     | 149.246&mu;s      | 17.299&mu;s       | **12.000ns**      | 166.557&mu;s      |
| 4     | 392.971&mu;s      | 141.498&mu;s      | **277.590&mu;s**  | 812.059&mu;s      |
| 5     | 449.636&mu;s      | 10.081ms          | 14.932ms          | 25.462ms          |
| 6     | 4.695&mu;s        | **307.000ns**     | **964.000ns**     | 5.966&mu;s        |
| 7     | 50.029&mu;s       | 409.000ns         | 1.194ms           | 1.245ms           |
| 8     | 88.161&mu;s       | 746.000ns         | 112.178&mu;s      | 201.085&mu;s      |
| 9     | 301.522&mu;s      | 698.693&mu;s      | 5.206ms           | 6.206ms           |
| 10    | **4.428&mu;s**    | 125.757&mu;s      | 129.746&mu;s      | 259.931&mu;s      |
| 11    | 14.447&mu;s       | 130.036&mu;s      | 360.259&mu;s      | 504.742&mu;s      |
| 12    | -                 | -                 | -                 | -                 |
| 13    | 386.040&mu;s      | 32.591&mu;s       | 213.612&mu;s      | 632.243&mu;s      |
| 14    | 6.610&mu;s        | 41.610&mu;s       | 168.608&mu;s      | 216.828&mu;s      |
| 15    | -                 | -                 | -                 | -                 |

### Rust

| Day   | Parse             | Part 1            | Part 2            | Total             |
|:-----:|:-----------------:|:-----------------:|:-----------------:|:-----------------:|
| 1     | **22.966&mu;s**   | **409.000ns**     | 3.739&mu;s        | **27.114&mu;s**   |
| 2     | 17.531&mu;s       | **706.000ns**     | **572.000ns**     | **18.809&mu;s**   |
| 3     | **54.638&mu;s**   | **5.954&mu;s**    | 76.451&mu;s       | **137.043&mu;s**  |
| 4     | **19.746&mu;s**   | **101.737&mu;s**  | 422.467&mu;s      | **543.950&mu;s**  |
| 5     | **22.846&mu;s**   | **1.147ms**       | **1.375ms**       | **2.545ms**       |
| 6     | **3.028&mu;s**    | 349.000ns         | 1.099&mu;s        | **4.476&mu;s**    |
| 7     | **19.012&mu;s**   | **209.000ns**     | **668.767&mu;s**  | **687.988&mu;s**  |
| 8     | **33.439&mu;s**   | **646.000ns**     | **9.991&mu;s**    | **44.076&mu;s**   |
| 9     | **14.723&mu;s**   | **7.174&mu;s**    | **1.051ms**       | **1.073ms**       |
| 10    | 49.995&mu;s       | **24.175&mu;s**   | **45.797&mu;s**   | **119.967&mu;s**  |
| 11    | **3.283&mu;s**    | **15.194&mu;s**   | **63.575&mu;s**   | **82.052&mu;s**   |
| 12    | **6.813&mu;s**    | **57.455&mu;s**   | **1.878ms**       | **1.942ms**       |
| 13    | **29.993&mu;s**   | **7.872&mu;s**    | **54.879&mu;s**   | **92.744&mu;s**   |
| 14    | **5.382&mu;s**    | **20.536&mu;s**   | **80.315&mu;s**   | **106.233&mu;s**  |
| 15    | **90.814&mu;s**   | **1.507ms**       | **128.002ms**     | **129.600ms**     |

### TypeScript (Deno)

| Day   | Parse             | Part 1            | Part 2            | Total             |
|:-----:|:-----------------:|:-----------------:|:-----------------:|:-----------------:|
| 1     | 98.800&mu;s       | 16.400&mu;s       | 65.200&mu;s       | 180.400&mu;s      |
| 2     | 158.000&mu;s      | 19.200&mu;s       | 10.800&mu;s       | 188.000&mu;s      |
| 3     | 194.400&mu;s      | 174.400&mu;s      | 21.200&mu;s       | 390.000&mu;s      |
| 4     | 297.200&mu;s      | 3.090ms           | 2.896ms           | 6.283ms           |
| 5     | 331.200&mu;s      | 71.214ms          | 53.928ms          | 125.473ms         |
| 6     | 15.600&mu;s       | 4.000&mu;s        | 3.200&mu;s        | 22.800&mu;s       |
| 7     | 161.200&mu;s      | 5.600&mu;s        | 3.116ms           | 3.283ms           |
| 8     | 168.800&mu;s      | 9.200&mu;s        | 631.600&mu;s      | 809.600&mu;s      |
| 9     | -                 | -                 | -                 | -                 |
| 10    | 13.600&mu;s       | 382.000&mu;s      | 206.000&mu;s      | 601.600&mu;s      |
| 11    | -                 | -                 | -                 | -                 |
| 12    | -                 | -                 | -                 | -                 |
| 13    | -                 | -                 | -                 | -                 |
| 14    | -                 | -                 | -                 | -                 |
| 15    | -                 | -                 | -                 | -                 |

### System Information

```sh
> go version
go version go1.17.5 linux/amd64

> rustc --version
rustc 1.59.0-nightly (404c8471a 2021-12-14)

> deno --version
deno 1.16.4 (release, x86_64-unknown-linux-gnu)
v8 9.7.106.15
typescript 4.4.2
```

All benchmarks were conducted on an Intel Core i7 12700k, with 32GB RAM and an NVMe SSD, using OpenSUSE Tumbleweed 20211213-0 via Windows 11's WSL.
