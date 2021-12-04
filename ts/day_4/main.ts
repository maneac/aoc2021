export interface Input {
  numbers: number[];
  boards: number[][];
}

export function readData(): Input {
  const input = Deno.readTextFileSync("./data/day_4.txt").trim();

  return parseContents(input);
}

export function parseContents(input: string): Input {
  const inputParts = input.split("\n\n");

  const numbers = inputParts[0].split(",").map((v) => +v);

  const boards = inputParts.slice(1).reduce((boardsArr, boardStr) => {
    boardsArr.push(
      boardStr.split("\n").reduce((boardRows, rowStr) => {
        rowStr.trim().split(" ").filter((v) => v.length > 0)
          .map((v) => +v).forEach((v) => boardRows.push(v));
        return boardRows;
      }, [] as number[]),
    );
    return boardsArr;
  }, [] as number[][]);

  return { numbers, boards };
}

export function part1(data: Input): number {
  for (const num of data.numbers) {
    for (const [boardIdx, board] of data.boards.entries()) {
      const newBoard = board.map((v) => v == num ? -1 : v);

      for (let rowIdx = 0; rowIdx < newBoard.length; rowIdx += 5) {
        if (newBoard.slice(rowIdx, rowIdx + 5).every((v) => v == -1)) {
          return newBoard.filter((v) => v >= 0).reduce((acc, v) => acc + v) *
            num;
        }
      }

      for (let colIdx = 0; colIdx < 5; colIdx++) {
        if (
          newBoard.filter((_, idx) => idx % 5 == colIdx).every((v) => v == -1)
        ) {
          return newBoard.filter((v) => v >= 0).reduce((acc, v) => acc + v) *
            num;
        }
      }

      data.boards[boardIdx] = newBoard;
    }
  }
  throw new Error("no result");
}

export function part2(data: Input): number {
  for (const num of data.numbers) {
    data.boards = data.boards.map((board) =>
      board.map((v) => v == num ? -1 : v)
    );

    const newBoards = data.boards.filter((newBoard) => {
      for (let rowIdx = 0; rowIdx < newBoard.length; rowIdx += 5) {
        if (newBoard.slice(rowIdx, rowIdx + 5).every((v) => v == -1)) {
          return false;
        }
      }

      for (let colIdx = 0; colIdx < 5; colIdx++) {
        if (
          newBoard.filter((_, idx) => idx % 5 == colIdx).every((v) => v == -1)
        ) {
          return false;
        }
      }
      return true;
    });

    if (data.boards.length == 1 && newBoards.length == 0) {
      return data.boards[0].filter((v) => v >= 0).reduce((acc, v) => acc + v) *
        num;
    }

    data.boards = newBoards;
  }
  throw new Error("no result");
}

export function main() {
  const data = readData();

  console.log("Part 1: ", part1(data));
  console.log("Part 2: ", part2(data));
}
