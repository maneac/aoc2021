package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type input struct {
	numbers []int
	boards  [][]int
}

func main() {
	data := readData()

	log.Println("Part 1: ", part1(data))
	log.Println("Part 2: ", part2(data))
}

func readData() *input {
	contents, err := os.ReadFile("../../data/day_4.txt")
	if err != nil {
		panic(err)
	}

	return parseContents(string(contents))
}

func parseContents(contents string) *input {
	contentParts := strings.Split(strings.TrimSpace(contents), "\n\n")
	rawNumbers := strings.Split(contentParts[0], ",")

	i := &input{
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

func part1(input *input) int {
	for _, num := range input.numbers {
		for _, board := range input.boards {
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
				return boardTotal * num
			}
		}
	}
	panic("no result")
}

func part2(input *input) int {
	winningBoards := make([]int, 0, len(input.boards))
	for _, num := range input.numbers {
		activeBoardCount := 0
		winningBoards = winningBoards[:0]
		for boardIdx, board := range input.boards {
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
			for _, val := range input.boards[winningBoards[0]] {
				if val < 0 {
					continue
				}
				boardTotal += val
			}
			return boardTotal * num
		}

		for _, idx := range winningBoards {
			input.boards[idx] = nil
		}
	}
	panic("no result")
}
