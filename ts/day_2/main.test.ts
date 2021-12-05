import { assertEquals } from "https://deno.land/std@0.116.0/testing/asserts.ts";
import * as day from "./main.ts";

Deno.test("part 1 real", () => {
  const input = day.readData();

  assertEquals(day.part1(input), 1840243);
});

Deno.test("part 2 real", () => {
  const input = day.readData();

  assertEquals(day.part2(input), 1727785422);
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
