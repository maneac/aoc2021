package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"

	"github.com/maneac/aoc2021/utils/lib/go/bench"
)

const (
	part1Solution = "8580"
	part2Solution = "9576"
)

type Input struct {
	numbers []int
	boards  [][]int
}

func main() {
	bench.Config{
		Filename:      "./bench/results/go/day_4.csv",
		DataDirectory: "./data",
		ReadData:      func(p string) bench.Day { return readData(p) },
		Part1Solution: part1Solution,
		Part2Solution: part2Solution,
	}.Run()
}

func readData(dir string) Input {
	contents, err := os.ReadFile(filepath.Join(dir, "day_4.txt"))
	if err != nil {
		panic(err)
	}

	return parseContents(string(contents))
}

func parseContents(contents string) Input {
	contentParts := strings.Split(strings.TrimSpace(contents), "\n\n")
	rawNumbers := strings.Split(contentParts[0], ",")

	i := Input{
		numbers: make([]int, 0, len(rawNumbers)),
		boards:  make([][]int, 0, len(contentParts)-1),
	}

	for _, rawNum := range rawNumbers {
		num, err := strconv.Atoi(rawNum)
		if err != nil {
			panic(err)
		}
		i.numbers = append(i.numbers, num)
	}

	for _, rawBoard := range contentParts[1:] {
		board := make([]int, 25)
		for rowOffset, row := range strings.Split(rawBoard, "\n") {
			idx := 5 * rowOffset
			_, err := fmt.Sscan(row, &board[idx], &board[idx+1], &board[idx+2], &board[idx+3], &board[idx+4])
			if err != nil {
				panic(err)
			}
		}
		i.boards = append(i.boards, board)
	}

	return i
}

func (i Input) Part1() string {
	for _, num := range i.numbers {
		for _, board := range i.boards {
			for idx, val := range board {
				if val == num {
					board[idx] = -1
				}
			}

			rowWon := false
			for row := 0; row < len(board); row += 5 {
				total := 0
				for col := 0; col < 5; col++ {
					total += board[row+col]
				}
				if total == -5 {
					rowWon = true
					break
				}
			}

			colWon := false
			for col := 0; col < 5; col++ {
				total := 0
				for row := 0; row < len(board); row += 5 {
					total += board[row+col]
				}
				if total == -5 {
					colWon = true
					break
				}
			}

			if rowWon || colWon {
				boardTotal := 0
				for _, val := range board {
					if val < 0 {
						continue
					}
					boardTotal += val
				}
				return fmt.Sprint(boardTotal * num)
			}
		}
	}
	panic("no result")
}

func (i Input) Part2() string {
	winningBoards := make([]int, 0, len(i.boards))
	for _, num := range i.numbers {
		activeBoardCount := 0
		winningBoards = winningBoards[:0]
		for boardIdx, board := range i.boards {
			if board == nil {
				continue
			}

			activeBoardCount++
			for idx, val := range board {
				if val == num {
					board[idx] = -1
				}
			}

			rowWon := false
			for row := 0; row < len(board); row += 5 {
				total := 0
				for col := 0; col < 5; col++ {
					total += board[row+col]
				}
				if total == -5 {
					rowWon = true
					break
				}
			}

			colWon := false
			for col := 0; col < 5; col++ {
				total := 0
				for row := 0; row < len(board); row += 5 {
					total += board[row+col]
				}
				if total == -5 {
					colWon = true
					break
				}
			}

			if rowWon || colWon {
				winningBoards = append(winningBoards, boardIdx)
			}
		}

		if activeBoardCount == 1 && len(winningBoards) == 1 {
			boardTotal := 0
			for _, val := range i.boards[winningBoards[0]] {
				if val < 0 {
					continue
				}
				boardTotal += val
			}
			return fmt.Sprint(boardTotal * num)
		}

		for _, idx := range winningBoards {
			i.boards[idx] = nil
		}
	}
	panic("no result")
}
