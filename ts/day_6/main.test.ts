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

const part1Solution = 360761;
const part2Solution = 1632779838045;

Deno.test("part 1 real", () => {
  const input = day.readData();

  assertEquals(day.part1(input), part1Solution);
});

Deno.test("part 2 real", () => {
  const input = day.readData();

  assertEquals(day.part2(input), part2Solution);
});

Deno.test("parse contents", () => {
  const input = "3,4,3,1,2";

  assertEquals(day.parseContents(input), [0, 1, 1, 2, 1, 0, 0, 0, 0]);
});

Deno.test("part 1 example", () => {
  const input: day.fish = [0, 1, 1, 2, 1, 0, 0, 0, 0];

  assertEquals(day.part1(input), 5934);
});

Deno.test("part 2 example", () => {
  const input: day.fish = [0, 1, 1, 2, 1, 0, 0, 0, 0];

  assertEquals(day.part2(input), 26984457539);
});

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
