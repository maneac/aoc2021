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

const part1Solution = 266301;
const part2Solution = 3404870164;

Deno.test("part 1 real", () => {
  const input = day.readData();

  assertEquals(day.part1(input), part1Solution);
});

Deno.test("part 2 real", () => {
  const input = day.readData();

  assertEquals(day.part2(input), part2Solution);
});

Deno.test("parse contents", () => {
  const input = `[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]`;

  assertEquals(day.parseContents(input), exampleData());
});

Deno.test("part 1 example", () => {
  const input = exampleData();

  assertEquals(day.part1(input), 26397);
});

Deno.test("part 2 example", () => {
  const input = exampleData();

  assertEquals(day.part2(input), 288957);
});

const exampleData = () => [
  "[({(<(())[]>[[{[]{<()<>>",
  "[(()[<>])]({[<{<<[]>>(",
  "{([(<{}[<>[]}>{[]{[(<()>",
  "(((({<>}<{<{<>}{[]{[]{}",
  "[[<[([]))<([[{}[[()]]]",
  "[{[{({}]{}}([{[{{{}}([]",
  "{<[[]]>}<{[{[{[]{()[[[]",
  "[<(<(<(<{}))><([]([]()",
  "<{([([[(<>()){}]>(<<{{",
  "<{([{{}}[<[[[<>{}]]]>[]]",
];

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
