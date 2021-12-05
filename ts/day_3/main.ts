function readData(): number[][] {
  return Deno.readTextFileSync("./data/day_3.txt").trim().split("\n").map(
    (line) => {
      return [...line].map((v) => +v);
    },
  );
}

function part1(data: number[][]): number {
  const gamma = data.reduce((acc, cur) => {
    return acc.map((old, idx) => old + cur[idx]);
  }).reduce((acc, c) => {
    acc *= 2;
    acc |= Number((c * 2) >= data.length);
    return acc;
  }, 0);

  return gamma * (~gamma & ((2 ** data[0].length) - 1));
}

function part2(data: number[][]): number {
  let oxygenFilter = data;
  for (let idx = 0; idx < data[0].length; idx++) {
    if (oxygenFilter.length < 2) {
      break;
    }
    const target = Number(
      oxygenFilter.reduce((acc, val) => {
            return acc + val[idx];
          }, 0) * 2 >= oxygenFilter.length,
    );
    oxygenFilter = oxygenFilter.filter((val) => {
      return val[idx] == target;
    });
  }

  let carbonFilter = data;
  for (let idx = 0; idx < data[0].length; idx++) {
    if (carbonFilter.length < 2) {
      break;
    }
    const target = Number(
      (carbonFilter.reduce((acc, val) => {
        return acc + val[idx];
      }, 0) * 2) < carbonFilter.length,
    );
    carbonFilter = carbonFilter.filter((val) => {
      return val[idx] == target;
    });
  }

  const oxygenRating = oxygenFilter[0].reduce((acc, val) => {
    return (acc << 1) | val;
  });

  const carbonRating = carbonFilter[0].reduce((acc, val) => {
    return (acc << 1) | val;
  });

  return oxygenRating * carbonRating;
}

function main() {
  const data = readData();

  console.log("Part 1: ", part1(data));
  console.log("Part 2: ", part2(data));
}

export { main, part1, part2, readData };
