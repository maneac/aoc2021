const readData = (): Display[] =>
  parseContents(Deno.readTextFileSync("./data/day_8.txt").trim());

const parseContents = (contents: string): Display[] =>
  contents.split("\n").map((line) => new Display().fromString(line)).reduce(
    (acc, d) => {
      acc.push(d);
      return acc;
    },
    [] as Display[],
  );

const part1 = (data: Display[]): number =>
  data.map((display) =>
    display.value.filter((v) =>
      v.length == 2 || v.length == 3 || v.length == 4 || v.length == 7
    )
      .length
  ).reduce((acc, cur) => acc + cur);

const part2 = (data: Display[]): number =>
  data.map((display) => display.decode()).reduce((acc, cur) => acc + cur);

function main() {
  const data = readData();

  console.log("Part 1: ", part1(data));
  console.log("Part 2: ", part2(data));
}

const NUMS: Record<string, number> = {
  "abcefg": 0,
  "cf": 1,
  "acdeg": 2,
  "acdfg": 3,
  "bcdf": 4,
  "abdfg": 5,
  "abdefg": 6,
  "acf": 7,
  "abcdefg": 8,
  "abcdfg": 9,
};

type digits = [
  string,
  string,
  string,
  string,
  string,
  string,
  string,
  string,
  string,
  string,
];

type values = [string, string, string, string];

class Display {
  digits: digits;
  value: values;

  constructor() {
    this.digits = ["", "", "", "", "", "", "", "", "", ""];
    this.value = ["", "", "", ""];
  }

  fromString(line: string): Display {
    const [digits, values] = line.split(" | ");
    digits.split(" ").forEach((digit, idx) => this.digits[idx] = digit);
    values.split(" ").forEach((value, idx) => this.value[idx] = value);
    return this;
  }

  fromComponents(digits: digits, value: values): Display {
    this.digits = digits;
    this.value = value;
    return this;
  }

  decode(): number {
    const mapping: Map<string, string> = new Map();

    const one = this.digits.find((digit) => digit.length == 2)?.split(
      "",
    ) as string[];
    const seven = this.digits.find((digit) => digit.length == 3)?.split(
      "",
    ) as string[];
    const four = this.digits.find((digit) => digit.length == 4)?.split(
      "",
    ) as string[];

    const a = seven.find((char) => !one.includes(char)) as string;
    mapping.set(a, "a");

    const lenFives = this.digits.filter((digit) => digit.length == 5).map((
      digit,
    ) => digit.split(""));

    const three = lenFives.find((digit) =>
      one.every((char) => digit.includes(char))
    ) as string[];

    const fourAndThree = four.filter((char) => three.includes(char));

    const b = four.find((char) => !fourAndThree.includes(char)) as string;
    mapping.set(b, "b");

    const g = three.find((char) =>
      !fourAndThree.includes(char) && char != a
    ) as string;
    mapping.set(g, "g");

    const five = lenFives.find((digit) => digit.includes(b)) as string[];
    const two = lenFives.find((digit) =>
      digit != five && digit != three
    ) as string[];

    const threeAndFive = three.filter((char) => five.includes(char));

    const c = three.find((char) => !threeAndFive.includes(char)) as string;
    mapping.set(c, "c");

    const twoAndThree = three.filter((char) => two.includes(char));

    const e = two.find((char) => !twoAndThree.includes(char)) as string;
    mapping.set(e, "e");

    const f = three.find((char) => !twoAndThree.includes(char)) as string;
    mapping.set(f, "f");

    const d = ["a", "b", "c", "d", "e", "f", "g"].find((char) =>
      !mapping.has(char)
    ) as string;
    mapping.set(d, "d");

    return this.value.map((v) => {
      return NUMS[v.split("").map((char) => mapping.get(char)).sort().join("")];
    }).reduce((acc, cur) => acc * 10 + cur);
  }
}

export { main, parseContents, part1, part2, readData };
export { Display };
