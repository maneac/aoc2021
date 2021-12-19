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
    <td>1</td>
    <td>+8%</td>
    <td>+45%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>+149%</td>
    <td><b>+0%</b></td>
    <td>+331%</td>
    <td>+4,923%</td>
    <td>+5,279%</td>
    <td><b>+640%</b></td>
  </tr>
  <tr>
    <td>2</td>
    <td>0%</td>
    <td>+58,208%</td>
    <td>+57,874%</td>
    <td><b>+3,497%</b></td>
    <td>+66%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>+1,391%</td>
    <td>+4,183%</td>
    <td>+1,782%</td>
    <td><b>+931%</b></td>
  </tr>
  <tr>
    <td>3</td>
    <td>+172%</td>
    <td>+189%</td>
    <td>0%</td>
    <td><b>+21%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>+653,467%</td>
    <td><b>0%</b></td>
    <td>+261%</td>
    <td>+2,702%</td>
    <td>+246,567%</td>
    <td><b>+186%</b></td>
  </tr>
  <tr>
    <td>4</td>
    <td>+1,864%</td>
    <td>+40%</td>
    <td>0%</td>
    <td><b>+48%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>+55%</td>
    <td><b>0%</b></td>
    <td>+1,394%</td>
    <td>+2,977%</td>
    <td>+951%</td>
    <td><b>+1,045%</b></td>
  </tr>
  <tr>
    <td>5</td>
    <td>+1,827%</td>
    <td>+683%</td>
    <td>+850%</td>
    <td><b>+783%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>+1,314%</td>
    <td>+5,360%</td>
    <td>+3,283%</td>
    <td><b>+4,196%</b></td>
  </tr>
  <tr>
    <td>6</td>
    <td>+57%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>+34%</b></td>
    <td>0%</td>
    <td>+14%</td>
    <td>+15%</td>
    <td><b>0%</b></td>
    <td>+424%</td>
    <td>+408%</td>
    <td>+471%</td>
    <td><b>+411%</b></td>
  </tr>
  <tr>
    <td>7</td>
    <td>+183%</td>
    <td>+93%</td>
    <td>+71%</td>
    <td><b>+74%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>+834%</td>
    <td>+4,219%</td>
    <td>+345%</td>
    <td><b>+358%</b></td>
  </tr>
  <tr>
    <td>8</td>
    <td>+164%</td>
    <td>+16%</td>
    <td>+1,035%</td>
    <td><b>+359%</b></td>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>+416%</td>
    <td>+1,114%</td>
    <td>+6,126%</td>
    <td><b>+1,720%</b></td>
  </tr>
  <tr>
    <td>9</td>
    <td>+2,028%</td>
    <td>+10,302%</td>
    <td>+385%</td>
    <td><b>+469%</b></td>
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
    <td>+422%</td>
    <td>+177%</td>
    <td><b>+114%</b></td>
    <td>+1,068%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>+225%</td>
    <td>+1,457%</td>
    <td>+346%</td>
    <td><b>+393%</b></td>
  </tr>
  <tr>
    <td>11</td>
    <td>+330%</td>
    <td>+701%</td>
    <td>+439%</td>
    <td><b>+484%</b></td>
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
    <td>+1,239%</td>
    <td>+329%</td>
    <td>+295%</td>
    <td><b>+601%</b></td>
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
    <td>+22%</td>
    <td>+102%</td>
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
  <tr>
    <td>16</td>
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
    <td>17</td>
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
    <td>18</td>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
    <td><b>0%</b></td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td><b>-</b></td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td><b>-</b></td>
  </tr>
  <tr>
    <td>19</td>
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
    <td>20</td>
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
| 1     | 25.770&mu;s       | 636ns             | **1.532&mu;s**    | **27.938&mu;s**   |
| 2     | **10.922&mu;s**   | 337.602&mu;s      | 344.945&mu;s      | 693.469&mu;s      |
| 3     | 153.148&mu;s      | 17.492&mu;s       | **12ns**          | 170.652&mu;s      |
| 4     | 401.338&mu;s      | 143.406&mu;s      | **281.203&mu;s**  | 825.947&mu;s      |
| 5     | 458.551&mu;s      | 10.482ms          | 15.473ms          | 26.414ms          |
| 6     | 4.797&mu;s        | **315ns**         | **981ns**         | 6.093&mu;s        |
| 7     | 51.217&mu;s       | 412ns             | 1.222ms           | 1.273ms           |
| 8     | 90.188&mu;s       | 765ns             | 115.718&mu;s      | 206.671&mu;s      |
| 9     | 311.062&mu;s      | 714.233&mu;s      | 5.250ms           | 6.275ms           |
| 10    | **4.429&mu;s**    | 129.334&mu;s      | 133.221&mu;s      | 266.984&mu;s      |
| 11    | 14.652&mu;s       | 131.069&mu;s      | 364.916&mu;s      | 510.637&mu;s      |
| 12    | -                 | -                 | -                 | -                 |
| 13    | 398.289&mu;s      | 33.182&mu;s       | 218.406&mu;s      | 649.877&mu;s      |
| 14    | 6.707&mu;s        | 42.030&mu;s       | 169.276&mu;s      | 218.013&mu;s      |
| 15    | -                 | -                 | -                 | -                 |
| 16    | -                 | -                 | -                 | -                 |
| 17    | -                 | -                 | -                 | -                 |
| 18    | **42.601&mu;s**   | **941.299&mu;s**  | **27.221ms**      | **28.205ms**      |
| 19    | -                 | -                 | -                 | -                 |
| 20    | -                 | -                 | -                 | -                 |

### Rust

| Day   | Parse             | Part 1            | Part 2            | Total             |
|:-----:|:-----------------:|:-----------------:|:-----------------:|:-----------------:|
| 1     | **23.755&mu;s**   | **438ns**         | 3.820&mu;s        | 28.013&mu;s       |
| 2     | 18.107&mu;s       | **579ns**         | **595ns**         | **19.281&mu;s**   |
| 3     | **56.325&mu;s**   | **6.052&mu;s**    | 78.428&mu;s       | **140.805&mu;s**  |
| 4     | **20.431&mu;s**   | **102.445&mu;s**  | 436.882&mu;s      | **559.758&mu;s**  |
| 5     | **23.797&mu;s**   | **1.338ms**       | **1.628ms**       | **2.990ms**       |
| 6     | **3.051&mu;s**    | 358ns             | 1.133&mu;s        | **4.542&mu;s**    |
| 7     | **18.077&mu;s**   | **213ns**         | **715.441&mu;s**  | **733.731&mu;s**  |
| 8     | **34.168&mu;s**   | **659ns**         | **10.196&mu;s**   | **45.023&mu;s**   |
| 9     | **14.616&mu;s**   | **6.866&mu;s**    | **1.081ms**       | **1.103ms**       |
| 10    | 51.720&mu;s       | **24.768&mu;s**   | **48.110&mu;s**   | **124.598&mu;s**  |
| 11    | **3.410&mu;s**    | **16.361&mu;s**   | **67.668&mu;s**   | **87.439&mu;s**   |
| 12    | **5.510&mu;s**    | **61.572&mu;s**   | **1.909ms**       | **1.977ms**       |
| 13    | **29.748&mu;s**   | **7.735&mu;s**    | **55.245&mu;s**   | **92.728&mu;s**   |
| 14    | **5.503&mu;s**    | **20.843&mu;s**   | **80.757&mu;s**   | **107.103&mu;s**  |
| 15    | **86.194&mu;s**   | **1.956ms**       | **130.886ms**     | **132.929ms**     |
| 16    | **15.386&mu;s**   | **273ns**         | **422ns**         | **16.081&mu;s**   |
| 17    | **1.149&mu;s**    | **0ns**           | **157.311&mu;s**  | **158.460&mu;s**  |
| 18    | -                 | -                 | -                 | -                 |
| 19    | **50.345&mu;s**   | **3.840s**        | **3.875s**        | **7.715s**        |
| 20    | **50.088&mu;s**   | **94.455&mu;s**   | **8.234ms**       | **8.379ms**       |

### TypeScript (Deno)

| Day   | Parse             | Part 1            | Part 2            | Total             |
|:-----:|:-----------------:|:-----------------:|:-----------------:|:-----------------:|
| 1     | 102.400&mu;s      | 22&mu;s           | 82.400&mu;s       | 206.800&mu;s      |
| 2     | 162.800&mu;s      | 24.800&mu;s       | 11.200&mu;s       | 198.800&mu;s      |
| 3     | 203.200&mu;s      | 169.600&mu;s      | 29.600&mu;s       | 402.400&mu;s      |
| 4     | 305.200&mu;s      | 3.152ms           | 2.954ms           | 6.412ms           |
| 5     | 336.400&mu;s      | 73.050ms          | 55.074ms          | 128.460ms         |
| 6     | 16&mu;s           | 1.600&mu;s        | 5.600&mu;s        | 23.200&mu;s       |
| 7     | 168.800&mu;s      | 9.200&mu;s        | 3.184ms           | 3.362ms           |
| 8     | 176.400&mu;s      | 8&mu;s            | 634.800&mu;s      | 819.200&mu;s      |
| 9     | -                 | -                 | -                 | -                 |
| 10    | 14.400&mu;s       | 385.600&mu;s      | 214.400&mu;s      | 614.400&mu;s      |
| 11    | -                 | -                 | -                 | -                 |
| 12    | -                 | -                 | -                 | -                 |
| 13    | -                 | -                 | -                 | -                 |
| 14    | -                 | -                 | -                 | -                 |
| 15    | -                 | -                 | -                 | -                 |
| 16    | -                 | -                 | -                 | -                 |
| 17    | -                 | -                 | -                 | -                 |
| 18    | -                 | -                 | -                 | -                 |
| 19    | -                 | -                 | -                 | -                 |
| 20    | -                 | -                 | -                 | -                 |

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
