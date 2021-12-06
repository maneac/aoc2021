import { assertEquals } from "https://deno.land/std@0.116.0/testing/asserts.ts";
import * as day from "./main.ts";

Deno.test("part 1 real", () => {
  const input = day.readData();

  assertEquals(day.part1(input), 360761);
});

Deno.test("part 2 real", () => {
  const input = day.readData();

  assertEquals(day.part2(input), 1632779838045);
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
