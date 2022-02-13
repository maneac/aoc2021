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

const part1Solution = 5608;
const part2Solution = 20299;

Deno.test("read data actual", () => {
  const input = day.readData();

  assertNotEquals(input, []);
});

Deno.test("parse contents example", () => {
  const input = `0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2`;

  assertEquals(day.parseContents(input), exampleData());
});

Deno.test("part 1 example", () => {
  const input = exampleData();

  assertEquals(day.part1(input), 5);
});

Deno.test("part 1 actual", () => {
  const input = day.readData();

  assertEquals(day.part1(input), part1Solution);
});
Deno.test("part 2 example", () => {
  const input = exampleData();

  assertEquals(day.part2(input), 12);
});

Deno.test("part 2 actual", () => {
  const input = day.readData();

  assertEquals(day.part2(input), part2Solution);
});

function exampleData(): day.Vector[] {
  return [
    new day.Vector(new day.Point(0, 9), new day.Point(5, 9)),
    new day.Vector(new day.Point(8, 0), new day.Point(0, 8)),
    new day.Vector(new day.Point(9, 4), new day.Point(3, 4)),
    new day.Vector(new day.Point(2, 2), new day.Point(2, 1)),
    new day.Vector(new day.Point(7, 0), new day.Point(7, 4)),
    new day.Vector(new day.Point(6, 4), new day.Point(2, 0)),
    new day.Vector(new day.Point(0, 9), new day.Point(2, 9)),
    new day.Vector(new day.Point(3, 4), new day.Point(1, 4)),
    new day.Vector(new day.Point(0, 0), new day.Point(8, 8)),
    new day.Vector(new day.Point(5, 5), new day.Point(8, 2)),
  ];
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
  runs: 1000,
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
  runs: 1000,
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
