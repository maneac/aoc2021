export type Input = Vector[];

export function readData(): Input {
  const contents = Deno.readTextFileSync("./data/day_5.txt").trim();

  return parseContents(contents);
}

export function parseContents(contents: string): Input {
  return contents.split("\n").map(
    (line) => {
      const fromTo = line.split(" -> ").map((point) => {
        const parts = point.split(",").map((v) => +v);
        return new Point(parts[0], parts[1]);
      });
      return new Vector(fromTo[0], fromTo[1]);
    },
  );
}

export function part1(data: Input): number {
  return Object.entries(
    data.filter(({ from, to }) => from.x == to.x || from.y == to.y).reduce(
      (acc, { from, to }) => {
        for (let x = Math.min(from.x, to.x); x <= Math.max(from.x, to.x); x++) {
          for (
            let y = Math.min(from.y, to.y);
            y <= Math.max(from.y, to.y);
            y++
          ) {
            const idx = (x << 16) + y;
            acc[idx] ? acc[idx]++ : acc[idx] = 1;
          }
        }
        return acc;
      },
      {} as Record<number, number>,
    ),
  ).filter(([_, count]) => count > 1).length;
}

export function part2(data: Input): number {
  return Object.entries(
    data.reduce((acc, { from, to }) => {
      if (from.x == to.x || from.y == to.y) {
        for (let x = Math.min(from.x, to.x); x <= Math.max(from.x, to.x); x++) {
          for (
            let y = Math.min(from.y, to.y);
            y <= Math.max(from.y, to.y);
            y++
          ) {
            const idx = (x << 16) + y;
            acc[idx] ? acc[idx]++ : acc[idx] = 1;
          }
        }
        return acc;
      }

      const increment = {
        x: (to.x - from.x) / (Math.abs(to.x - from.x)),
        y: (to.y - from.y) / (Math.abs(to.y - from.y)),
      };
      for (let i = 0; i <= Math.abs(to.x - from.x); i++) {
        const idx = ((from.x + (i * increment.x)) << 16) +
          (from.y + (i * increment.y));
        acc[idx] ? acc[idx]++ : acc[idx] = 1;
      }
      return acc;
    }, {} as Record<number, number>),
  ).filter(([_, count]) => count > 1).length;
}

export class Point {
  x: number;
  y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
}

export class Vector {
  from: Point;
  to: Point;

  constructor(from: Point, to: Point) {
    this.from = from;
    this.to = to;
  }
}

export function main() {
  const data = readData();

  console.log("Part 1: ", part1(data));
  console.log("Part 2: ", part2(data));
}
