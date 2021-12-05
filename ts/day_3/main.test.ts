import { assertEquals } from "https://deno.land/std@0.116.0/testing/asserts.ts";
import * as day from "./main.ts";

Deno.test("part 1 real", () => {
  const input = day.readData();

  assertEquals(day.part1(input), 3148794);
});

Deno.test("part 2 real", () => {
  const input = day.readData();

  assertEquals(day.part2(input), 2795310);
});

Deno.test("part 1 example", () => {
  const input = [
    [0, 0, 1, 0, 0],
    [1, 1, 1, 1, 0],
    [1, 0, 1, 1, 0],
    [1, 0, 1, 1, 1],
    [1, 0, 1, 0, 1],
    [0, 1, 1, 1, 1],
    [0, 0, 1, 1, 1],
    [1, 1, 1, 0, 0],
    [1, 0, 0, 0, 0],
    [1, 1, 0, 0, 1],
    [0, 0, 0, 1, 0],
    [0, 1, 0, 1, 0],
  ];

  assertEquals(day.part1(input), 198);
});

Deno.test("part 2 example", () => {
  const input = [
    [0, 0, 1, 0, 0],
    [1, 1, 1, 1, 0],
    [1, 0, 1, 1, 0],
    [1, 0, 1, 1, 1],
    [1, 0, 1, 0, 1],
    [0, 1, 1, 1, 1],
    [0, 0, 1, 1, 1],
    [1, 1, 1, 0, 0],
    [1, 0, 0, 0, 0],
    [1, 1, 0, 0, 1],
    [0, 0, 0, 1, 0],
    [0, 1, 0, 1, 0],
  ];

  assertEquals(day.part2(input), 230);
});
