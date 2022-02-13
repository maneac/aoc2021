export type Input = Fish;

export function readData(): Input {
  const contents = Deno.readTextFileSync("./data/day_6.txt").trim();

  return parseContents(contents);
}

export function parseContents(contents: string): Input {
  return contents.split(",").reduce((acc, num) => {
    acc[+num]++;
    return acc;
  }, [0, 0, 0, 0, 0, 0, 0, 0, 0] as Fish);
}

export function part1(data: Input): number {
  return fishAfterDays(data, 80);
}

export function part2(data: Input): number {
  return fishAfterDays(data, 256);
}

function fishAfterDays(data: Fish, days: number): number {
  for (let i = 0; i < days; i++) {
    data.push(data.shift() as number);
    data[6] += data[8];
  }
  return data.reduce((acc, val) => acc + val);
}

export type Fish = [
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
];

export function main() {
  const data = readData();

  console.log("Part 1: ", part1(data));
  console.log("Part 2: ", part2(data));
}
