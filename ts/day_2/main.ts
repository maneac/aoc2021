export type Input = Instruction[];

export function readData(): Input {
  const contents = Deno.readTextFileSync("./data/day_2.txt").trim();

  return parseContents(contents);
}

export function parseContents(contents: string): Input {
  return contents.split("\n").map(
    (line) => {
      const lineParts = line.split(" ");
      if (lineParts.length != 2) {
        throw new Error("invalid line");
      }
      return new Instruction(
        lineParts[0],
        Number(lineParts[1]),
      );
    },
  );
}

export function part1(data: Input): number {
  let horizontal = 0;
  let depth = 0;
  for (const line of data) {
    switch (line.instruction) {
      case "forward":
        horizontal += line.amount;
        break;
      case "down":
        depth += line.amount;
        break;
      case "up":
        depth -= line.amount;
    }
  }
  return (horizontal * depth);
}

export function part2(data: Input): number {
  let horizontal = 0;
  let aim = 0;
  let depth = 0;
  for (const line of data) {
    switch (line.instruction) {
      case "forward":
        horizontal += line.amount;
        depth += aim * line.amount;
        break;
      case "down":
        aim += line.amount;
        break;
      case "up":
        aim -= line.amount;
    }
  }
  return (horizontal * depth);
}

export class Instruction {
  instruction: string;
  amount: number;

  constructor(instruction: string, amount: number) {
    this.instruction = instruction;
    this.amount = amount;
  }
}

export function main() {
  const data = readData();

  console.log("Part 1: ", part1(data));
  console.log("Part 2: ", part2(data));
}
