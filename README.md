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

| s = 1000ms, ms = 1000&mu;s, &mu;s = 1000ns ||||||||||
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:--:|
| **Day**   | **Go**                                            ||| **Rust**                                ||| **TypeScript (Deno)\***               |||
| ^         | **Parse**     | **Part 1**    | **Part 2**        | **Parse** | **Part 1**    | **Part 2**    | **Parse** | **Part 1**    | **Part 2**    |
|||||||||||
| 1         | 26.021&mu;s   | **640.2ns**   | **1.553&mu;s**    | **411ns** | 3.833&mu;s    | 22.965&mu;s   | 116&mu;s  | 36&mu;s       | 88&mu;s       |
| 2         | 10.943&mu;s   | 339.841&mu;s  | 341.544&mu;s      | **734ns** | **589ns**     | 17.61&mu;s    | 180&mu;s  | 32&mu;s       | **16&mu;s**   |
| 3         | -             | 0ms           | 0ms               | -         | 0ms           | 0ms           | -         | 0ms           | 0ms           |
| 4         | -             | 0ms           | 0ms               | -         | 0ms           | 0ms           | -         | 0ms           | 0ms           |
| 5         | -             | 0ms           | 0ms               | -         | 0ms           | 0ms           | -         | 0ms           | 0ms           |
| 6         | -             | 0ms           | 0ms               | -         | 0ms           | 0ms           | -         | 0ms           | 0ms           |
| 7         | -             | 0ms           | 0ms               | -         | 0ms           | 0ms           | -         | 0ms           | 0ms           |
| 8         | -             | 0ms           | 0ms               | -         | 0ms           | 0ms           | -         | 0ms           | 0ms           |

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
