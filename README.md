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

<centre>

| s = 1000ms, ms = 1000&mu;s, &mu;s = 1000ns                                                                                                                                                                                          |||||||||||||||
|:---------:|:-----------------:|:-----------------:|:-----------------:|:-----------------:|:-----:|:-----------------:|:-----------------:|:-----------------:|:-----------------:|:-----:|:---------:|:-------------:|:-------------:|:---------:|
| **Day**   |                               **Go**                                       ||||&nbsp; |                            **Rust**                                        ||||&nbsp; |            **TypeScript (Deno)\***                    |
| ^         | **Parse**         | **Part 1**        | **Part 2**        | **Total**         |&nbsp; | **Parse**         | **Part 1**        | **Part 2**        | **Total**         |&nbsp; | **Parse** | **Part 1**    | **Part 2**    | **Total** |
| 1         | 26.021&mu;s       | 640.2ns           | **1.553&mu;s**    | 28.214&mu;s       |&nbsp; | **22.965&mu;s**   | **411ns**         | 3.833&mu;s        | **27.209&mu;s**   |&nbsp; | 116&mu;s  | 36&mu;s       | 88&mu;s       | 240&mu;s  |
| 2         | **10.943&mu;s**   | 339.841&mu;s      | 341.544&mu;s      | 692.328&mu;s      |&nbsp; | 17.61&mu;s        | **734ns**         | **589ns**         | **18.933&mu;s**   |&nbsp; | 180&mu;s  | 32&mu;s       | 16&mu;s       | 228&mu;s  |
| 3         | *154.031&mu;s*    | *17.639&mu;s*     | ***11.98ns***     | *171.682&mu;s*    |&nbsp; | **55.775&mu;s**   | **6.083&mu;s**    | 80.437&mu;s       | **142.295&mu;s**  |&nbsp; | 224&mu;s  | 216&mu;s      | 52&mu;s       | 492&mu;s  |
| 4         | 403.665&mu;s      | 144.429&mu;s      | **286.581&mu;s**  | 834.675&mu;s      |&nbsp; | **20.331&mu;s**   | **104.342&mu;s**  | 428.096&mu;s      | **552.769&mu;s**  |&nbsp; | 320&mu;s  | 3.132ms       | 2.936ms       | 6.388ms   |
| 5         | 461.056&mu;s      | 10.627ms          | 15.657ms          | 26.746ms          |&nbsp; | **25.155&mu;s**   | **1.275ms**       | **1.548ms**       | **2.848ms**       |&nbsp; | 348&mu;s  | 72.576ms      | 54.76ms       | 127.684ms |
| 6         | -                 | 0ms               | 0ms               | -                 |&nbsp; | -                 | 0ms               | 0ms               | -                 |&nbsp; | -         | 0ms           | 0ms           | -         |
| 7         | -                 | 0ms               | 0ms               | -                 |&nbsp; | -                 | 0ms               | 0ms               | -                 |&nbsp; | -         | 0ms           | 0ms           | -         |
| 8         | -                 | 0ms               | 0ms               | -                 |&nbsp; | -                 | 0ms               | 0ms               | -                 |&nbsp; | -         | 0ms           | 0ms           | -         |

</centre>

### Key

- **bold** - fastest language for that day / part.
- *italics* - different implementation, not necessarily comparable.

\* Due to framework limitations, all TypeScript benchmark results are measured to the nearest &mu;s.

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

All benchmarks were conducted on an Intel Core i7 12700k, with 32GB RAM and an NVMe SSD, using OpenSUSE Tubmleweed 20211207-0 via Windows 11's WSL.
