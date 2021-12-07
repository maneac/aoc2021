const readData = (): number[] =>
  Deno.readTextFileSync("./data/day_7.txt").trim().split(",").map((v) => +v)
    .sort((a, b) => a - b);

const part1 = (data: number[]): number =>
  data.reduce((acc, cur) => acc + Math.abs(cur - data[data.length / 2]), 0);

function part2(data: number[]): number {
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

function main() {
  const data = readData();

  console.log("Part 1: ", part1(data));
  console.log("Part 2: ", part2(data));
}

export { main, part1, part2, readData };
