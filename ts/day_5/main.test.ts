import { assertEquals } from "https://deno.land/std@0.116.0/testing/asserts.ts";
import * as day from "./main.ts";

Deno.test("part 1 real", () => {
  const input = day.readData();

  assertEquals(day.part1(input), 5608);
});

Deno.test("part 2 real", () => {
  const input = day.readData();

  assertEquals(day.part2(input), 20299);
});

Deno.test("parse contents", () => {
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

  assertEquals(day.parseContents(input), exampleInput());
});

Deno.test("part 1 example", () => {
  const input = exampleInput();

  assertEquals(day.part1(input), 5);
});

Deno.test("part 2 example", () => {
  const input = exampleInput();

  assertEquals(day.part2(input), 12);
});

function exampleInput(): day.Vector[] {
  return [
    { from: { x: 0, y: 9 }, to: { x: 5, y: 9 } },
    { from: { x: 8, y: 0 }, to: { x: 0, y: 8 } },
    { from: { x: 9, y: 4 }, to: { x: 3, y: 4 } },
    { from: { x: 2, y: 2 }, to: { x: 2, y: 1 } },
    { from: { x: 7, y: 0 }, to: { x: 7, y: 4 } },
    { from: { x: 6, y: 4 }, to: { x: 2, y: 0 } },
    { from: { x: 0, y: 9 }, to: { x: 2, y: 9 } },
    { from: { x: 3, y: 4 }, to: { x: 1, y: 4 } },
    { from: { x: 0, y: 0 }, to: { x: 8, y: 8 } },
    { from: { x: 5, y: 5 }, to: { x: 8, y: 2 } },
  ];
}