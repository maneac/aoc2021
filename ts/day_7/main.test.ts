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

const part1Solution = 340056;
const part2Solution = 96592275;

Deno.test("read data actual", () => {
  const input = day.readData();

  assertNotEquals(input, []);
});

Deno.test("parse contents example", () => {
  const input = `16,1,2,0,4,2,7,1,2,14`;

  assertEquals(day.parseContents(input), exampleData());
});

Deno.test("part 1 example", () => {
  const input = exampleData();

  assertEquals(day.part1(input), 37);
});

Deno.test("part 1 actual", () => {
  const input = day.readData();

  assertEquals(day.part1(input), part1Solution);
});

Deno.test("part 2 example", () => {
  const input = exampleData();

  assertEquals(day.part2(input), 168);
});

Deno.test("part 2 actual", () => {
  const input = day.readData();

  assertEquals(day.part2(input), part2Solution);
});

function exampleData(): day.Input {
  return [0, 1, 1, 2, 2, 2, 4, 7, 14, 16];
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
