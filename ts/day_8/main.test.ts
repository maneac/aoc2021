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

const part1Solution = 284;
const part2Solution = 973499;

Deno.test("part 1 real", () => {
  const input = day.readData();

  assertEquals(day.part1(input), part1Solution);
});

Deno.test("part 2 real", () => {
  const input = day.readData();

  assertEquals(day.part2(input), part2Solution);
});

Deno.test("parse contents example 1", () => {
  const input =
    `acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf`;

  assertEquals(day.parseContents(input), example1Data());
});

Deno.test("parse contents example 2", () => {
  const input =
    `be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce`;

  assertEquals(day.parseContents(input), example2Data());
});

Deno.test("part 1 example 1", () => {
  const input = example1Data();

  assertEquals(day.part1(input), 0);
});

Deno.test("part 1 example 2", () => {
  const input = example2Data();

  assertEquals(day.part1(input), 26);
});

Deno.test("part 2 example 1", () => {
  const input = example1Data();

  assertEquals(day.part2(input), 5353);
});

Deno.test("part 2 example 2", () => {
  const input = example2Data();

  assertEquals(day.part2(input), 61229);
});

const example1Data = () => [
  new day.Display().fromComponents([
    "acedgfb",
    "cdfbe",
    "gcdfa",
    "fbcad",
    "dab",
    "cefabd",
    "cdfgeb",
    "eafb",
    "cagedb",
    "ab",
  ], ["cdfeb", "fcadb", "cdfeb", "cdbaf"]),
];

const example2Data = () => [
  new day.Display().fromComponents([
    "be",
    "cfbegad",
    "cbdgef",
    "fgaecd",
    "cgeb",
    "fdcge",
    "agebfd",
    "fecdb",
    "fabcd",
    "edb",
  ], ["fdgacbe", "cefdb", "cefbgd", "gcbe"]),
  new day.Display().fromComponents([
    "edbfga",
    "begcd",
    "cbg",
    "gc",
    "gcadebf",
    "fbgde",
    "acbgfd",
    "abcde",
    "gfcbed",
    "gfec",
  ], ["fcgedb", "cgb", "dgebacf", "gc"]),
  new day.Display().fromComponents([
    "fgaebd",
    "cg",
    "bdaec",
    "gdafb",
    "agbcfd",
    "gdcbef",
    "bgcad",
    "gfac",
    "gcb",
    "cdgabef",
  ], ["cg", "cg", "fdcagb", "cbg"]),
  new day.Display().fromComponents([
    "fbegcd",
    "cbd",
    "adcefb",
    "dageb",
    "afcb",
    "bc",
    "aefdc",
    "ecdab",
    "fgdeca",
    "fcdbega",
  ], ["efabcd", "cedba", "gadfec", "cb"]),
  new day.Display().fromComponents([
    "aecbfdg",
    "fbg",
    "gf",
    "bafeg",
    "dbefa",
    "fcge",
    "gcbea",
    "fcaegb",
    "dgceab",
    "fcbdga",
  ], ["gecf", "egdcabf", "bgf", "bfgea"]),
  new day.Display().fromComponents([
    "fgeab",
    "ca",
    "afcebg",
    "bdacfeg",
    "cfaedg",
    "gcfdb",
    "baec",
    "bfadeg",
    "bafgc",
    "acf",
  ], ["gebdcfa", "ecba", "ca", "fadegcb"]),
  new day.Display().fromComponents([
    "dbcfg",
    "fgd",
    "bdegcaf",
    "fgec",
    "aegbdf",
    "ecdfab",
    "fbedc",
    "dacgb",
    "gdcebf",
    "gf",
  ], ["cefg", "dcbef", "fcge", "gbcadfe"]),
  new day.Display().fromComponents([
    "bdfegc",
    "cbegaf",
    "gecbf",
    "dfcage",
    "bdacg",
    "ed",
    "bedf",
    "ced",
    "adcbefg",
    "gebcd",
  ], ["ed", "bcgafe", "cdgba", "cbgef"]),
  new day.Display().fromComponents([
    "egadfb",
    "cdbfeg",
    "cegd",
    "fecab",
    "cgb",
    "gbdefca",
    "cg",
    "fgcdab",
    "egfdb",
    "bfceg",
  ], ["gbdfcae", "bgc", "cg", "cgb"]),
  new day.Display().fromComponents([
    "gcafb",
    "gcf",
    "dcaebfg",
    "ecagb",
    "gf",
    "abcdeg",
    "gaef",
    "cafbge",
    "fdbac",
    "fegbdc",
  ], ["fgae", "cfgab", "fg", "bagce"]),
];

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

if (Deno.args.length > 0 && Deno.args[0] == "--bench") {
  runBenchmarks();
}
