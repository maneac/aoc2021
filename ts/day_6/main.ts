type fish = [
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

function readData(): fish {
  const input = Deno.readTextFileSync("./data/day_6.txt").trim();

  return parseContents(input);
}

const parseContents = (input: string): fish =>
  input.split(",").reduce((acc, num) => {
    acc[+num]++;
    return acc;
  }, [0, 0, 0, 0, 0, 0, 0, 0, 0]);

const part1 = (data: fish): number => fishAfterDays(data, 80);

const part2 = (data: fish): number => fishAfterDays(data, 256);

function fishAfterDays(data: fish, days: number): number {
  for (let i = 0; i < days; i++) {
    data.push(data.shift() as number);
    data[6] += data[8];
  }
  return data.reduce((acc, val) => acc + val);
}

function main() {
  const data = readData();

  console.log("Part 1: ", part1(data));
  console.log("Part 2: ", part2(data));
}

export { main, parseContents, part1, part2, readData };
export type { fish };
