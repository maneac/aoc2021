export type Input = number[];

export function readData(): Input {
  const contents = Deno.readTextFileSync("./data/day_1.txt").trim();

  return parseContents(contents);
}

export function parseContents(contents: string): Input {
  return contents.split("\n").map(
    (v) => +v,
  );
}

export function part1(data: Input): number {
  return data.reduce((acc, num, idx, arr) => {
    if (idx == 0) {
      return 0;
    }
    if (num > arr[idx - 1]) {
      acc++;
    }
    return acc;
  }, 0);
}

export function part2(data: Input): number {
  return data.reduce((acc, _, idx, arr) => {
    if (idx < 3) {
      return 0;
    }

    const lastSum = arr.slice(idx - 3, idx).reduce((innerAcc, num, _) =>
      innerAcc + num
    );
    const thisSum = arr.slice(idx - 2, idx + 1).reduce((innerAcc, num, _) =>
      innerAcc + num
    );

    if (thisSum > lastSum) {
      acc++;
    }
    return acc;
  }, 0);
}

export function main() {
  const data = readData();

  console.log("Part 1: ", part1(data));
  console.log("Part 2: ", part2(data));
}

main();
