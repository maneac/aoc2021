function readData(): string[] {
  const contents = Deno.readTextFileSync("./data/day_10.txt").trim();

  return parseContents(contents);
}

function parseContents(contents: string): string[] {
  return contents.split("\n");
}

function part1(data: string[]): number {
  return data.map((line): number => {
    try {
      fixLine(line);
    } catch (e) {
      switch (e) {
        case ")":
          return 3;
        case "]":
          return 57;
        case "}":
          return 1197;
        case ">":
          return 25137;
      }
    }
    return 0;
  }).reduce((acc, cur) => acc + cur);
}

function part2(data: string[]): number {
  const fixScores = data.map((line) => {
    try {
      const fix = fixLine(line);
      return fix.split("").reduce((acc, cur) => {
        switch (cur) {
          case ")":
            return (acc * 5) + 1;
          case "]":
            return (acc * 5) + 2;
          case "}":
            return (acc * 5) + 3;
          case ">":
            return (acc * 5) + 4;
          default:
            throw new Error("invalid character in fix: " + cur);
        }
      }, 0);
    } catch {
      return 0;
    }
  }).filter((fixScore) => fixScore > 0);

  return fixScores.sort((a, b) => a - b)[(fixScores.length - 1) / 2];
}

function fixLine(line: string): string {
  let missing = "";
  while (line.length > 0) {
    const chr = line[0];
    line = line.slice(1);

    switch (chr) {
      case "(":
        missing = ")" + missing;
        break;
      case "[":
        missing = "]" + missing;
        break;
      case "{":
        missing = "}" + missing;
        break;
      case "<":
        missing = ">" + missing;
        break;
      case ")":
      case "]":
      case "}":
      case ">": {
        const match = missing[0];
        missing = missing.slice(1);
        if (match != chr) {
          throw chr;
        }
        break;
      }
    }
  }
  return missing;
}

function main() {
  const data = readData();

  console.log("Part 1: ", part1(data));
  console.log("Part 2: ", part2(data));
}

export { main, parseContents, part1, part2, readData };
