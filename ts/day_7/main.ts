export type Input = number[];

export function readData(): Input {
  const contents = Deno.readTextFileSync("./data/day_7.txt").trim();

  return parseContents(contents);
}

export function parseContents(contents: string): Input {
  return contents.split(",").map((v) => +v)
    .sort((a, b) => a - b);
}

export function part1(data: Input): number {
  return data.reduce(
    (acc, cur) => acc + Math.abs(cur - data[data.length / 2]),
    0,
  );
}

export function part2(data: Input): number {
  let total = Number.MAX_SAFE_INTEGER;
  for (let target = 0; target < data[data.length - 1]; target++) {
    total = Math.min(
      total,
      data.reduce((acc, cur) => {
        const diff = Math.abs(cur - target);
        return acc + ((diff * (diff + 1)) / 2);
      }, 0),
    );
  }
  return total;
}

export function main() {
  const data = readData();

  console.log("Part 1: ", part1(data));
  console.log("Part 2: ", part2(data));
}
