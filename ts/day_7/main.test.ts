import { assertEquals } from "https://deno.land/std@0.116.0/testing/asserts.ts";
import * as day from "./main.ts";

Deno.test("part 1 real", () => {
  const input = day.readData();

  assertEquals(day.part1(input), 340056);
});

Deno.test("part 2 real", () => {
  const input = day.readData();

  assertEquals(day.part2(input), 96592275);
});

Deno.test("part 1 example", () => {
  const input = [0, 1, 1, 2, 2, 2, 4, 7, 14, 16];

  assertEquals(day.part1(input), 37);
});

Deno.test("part 2 example", () => {
  const input = [0, 1, 1, 2, 2, 2, 4, 7, 14, 16];

  assertEquals(day.part2(input), 168);
});
