import { assertEquals } from "https://deno.land/std@0.116.0/testing/asserts.ts";
import { part1, part2, readData } from "./main.ts";

Deno.test("part 1 real", () => {
  const input = readData();

  assertEquals(part1(input), 1374);
});

Deno.test("part 2 real", () => {
  const input = readData();

  assertEquals(part2(input), 1418);
});

Deno.test("part 1 example", () => {
  const input = [
    199,
    200,
    208,
    210,
    200,
    207,
    240,
    269,
    260,
    263,
  ];

  assertEquals(part1(input), 7);
});

Deno.test("part 2 example", () => {
  const input = [
    199,
    200,
    208,
    210,
    200,
    207,
    240,
    269,
    260,
    263,
  ];

  assertEquals(part2(input), 5);
});
