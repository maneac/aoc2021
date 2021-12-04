interface Instruction {
  instruction: string;
  amount: number;
}

export function readData(): Instruction[] {
  return Deno.readTextFileSync("./data/day_2.txt").trim().split("\n").map(
    (line) => {
      const lineParts = line.split(" ");
      if (lineParts.length != 2) {
        throw new Error("invalid line");
      }
      return {
        instruction: lineParts[0],
        amount: Number(lineParts[1]),
      };
    },
  );
}

export function part1(data: Instruction[]): number {
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

export function part2(data: Instruction[]): number {
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

export function main() {
  const data = readData();

  console.log("Part 1: ", part1(data));
  console.log("Part 2: ", part2(data));
}
