interface Point {
  x: number;
  y: number;
}

interface Vector {
  from: Point;
  to: Point;
}

function readData(): Vector[] {
  const input = Deno.readTextFileSync("./data/day_5.txt").trim();

  return parseContents(input);
}

function parseContents(input: string): Vector[] {
  return input.split("\n").map(
    (line) => {
      const fromTo = line.split(" -> ").map((point) => {
        const parts = point.split(",").map((v) => +v);
        return { x: parts[0], y: parts[1] };
      });
      return { from: fromTo[0], to: fromTo[1] };
    },
  );
}

function part1(data: Vector[]): number {
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

function part2(data: Vector[]): number {
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

function main() {
  const data = readData();

  console.log("Part 1: ", part1(data));
  console.log("Part 2: ", part2(data));
}

export { main, parseContents, part1, part2, readData };
export type { Vector };
