import {
  assertEquals,
  assertNotEquals,
} from "https://deno.land/std@0.116.0/testing/asserts.ts";
import {
  bench,
  BenchmarkTimer,
  runBenchmarks,
} from "https://deno.land/std@0.116.0/testing/bench.ts";
import * as day from "./main.ts";

const part1Solution = 1840243;
const part2Solution = 1727785422;

Deno.test("part 1 real", () => {
  const input = day.readData();

  assertEquals(day.part1(input), part1Solution);
});

Deno.test("part 2 real", () => {
  const input = day.readData();

  assertEquals(day.part2(input), part2Solution);
});

Deno.test("part 1 example", () => {
  const input = [
    { instruction: "forward", amount: 5 },
    { instruction: "down", amount: 5 },
    { instruction: "forward", amount: 8 },
    { instruction: "up", amount: 3 },
    { instruction: "down", amount: 8 },
    { instruction: "forward", amount: 2 },
  ];

  assertEquals(day.part1(input), 150);
});

Deno.test("part 2 example", () => {
  const input = [
    { instruction: "forward", amount: 5 },
    { instruction: "down", amount: 5 },
    { instruction: "forward", amount: 8 },
    { instruction: "up", amount: 3 },
    { instruction: "down", amount: 8 },
    { instruction: "forward", amount: 2 },
  ];

  assertEquals(day.part2(input), 900);
});

bench({
  name: "read data",
  runs: 500,
  func(b: BenchmarkTimer): void {
    b.start();
    const input = day.readData();
    assertNotEquals(input, []);
    b.stop();
  },
});

bench({
  name: "part 1",
  runs: 500,
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
  runs: 500,
  func(b: BenchmarkTimer): void {
    const input = day.readData();
    b.start();
    assertEquals(day.part2(input), part2Solution);
    b.stop();
  },
});

runBenchmarks();
