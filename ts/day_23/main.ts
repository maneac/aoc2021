function readData(): any {
  const _ = Deno.readTextFileSync("./data/day_23.txt").trim();
  throw new Error("unimplemented");
}

function part1(_data: any): number {
  throw new Error("unimplemented");
}

function part2(_data: any): number {
  throw new Error("unimplemented");
}

function main() {
  const data = readData();

  console.log("Part 1: ", part1(data));
  console.log("Part 2: ", part2(data));
}

export { main, part1, part2, readData };
