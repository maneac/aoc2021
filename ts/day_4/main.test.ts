import {
  assertEquals,
  assertNotEquals,
} from "https://deno.land/std@0.117.0/testing/asserts.ts";
import {
  bench,
  BenchmarkTimer,
  runBenchmarks,
} from "https://deno.land/std@0.117.0/testing/bench.ts";
import * as day from "./main.ts";

const part1Solution = 8580;
const part2Solution = 9576;

Deno.test("read data actual", () => {
  const input = day.readData();

  assertNotEquals(input, []);
});

Deno.test("parse contents example", () => {
  const input =
    `7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7`;

  assertEquals(day.parseContents(input), exampleData());
});

Deno.test("part 1 example", () => {
  const input = exampleData();

  assertEquals(day.part1(input), 4512);
});

Deno.test("part 1 actual", () => {
  const input = day.readData();

  assertEquals(day.part1(input), part1Solution);
});

Deno.test("part 2 example", () => {
  const input = exampleData();

  assertEquals(day.part2(input), 1924);
});

Deno.test("part 2 actual", () => {
  const input = day.readData();

  assertEquals(day.part2(input), part2Solution);
});

function exampleData(): day.Input {
  return new day.Input(
    [
      7,
      4,
      9,
      5,
      11,
      17,
      23,
      2,
      0,
      14,
      21,
      24,
      10,
      16,
      13,
      6,
      15,
      25,
      12,
      22,
      18,
      20,
      8,
      19,
      3,
      26,
      1,
    ],
    [
      [
        22,
        13,
        17,
        11,
        0,
        8,
        2,
        23,
        4,
        24,
        21,
        9,
        14,
        16,
        7,
        6,
        10,
        3,
        18,
        5,
        1,
        12,
        20,
        15,
        19,
      ],
      [
        3,
        15,
        0,
        2,
        22,
        9,
        18,
        13,
        17,
        5,
        19,
        8,
        7,
        25,
        23,
        20,
        11,
        10,
        24,
        4,
        14,
        21,
        16,
        12,
        6,
      ],
      [
        14,
        21,
        17,
        24,
        4,
        10,
        16,
        15,
        9,
        19,
        18,
        8,
        23,
        26,
        20,
        22,
        11,
        13,
        6,
        5,
        2,
        0,
        12,
        3,
        7,
      ],
    ],
  );
}

bench({
  name: "read data",
  runs: 5000,
  func(b: BenchmarkTimer): void {
    b.start();
    const input = day.readData();
    assertNotEquals(input, []);
    b.stop();
  },
});

bench({
  name: "part 1",
  runs: 5000,
  func(b: BenchmarkTimer): void {
    const input = day.readData();
    b.start();
    assertEquals(day.part1(input), part1Solution);
    day.part1(input);
    b.stop();
  },
});

bench({
  name: "part 2",
  runs: 5000,
  func(b: BenchmarkTimer): void {
    const input = day.readData();
    b.start();
    assertEquals(day.part2(input), part2Solution);
    b.stop();
  },
});

if (Deno.args.length > 0 && Deno.args[0] == "--bench") {
  runBenchmarks();
}
