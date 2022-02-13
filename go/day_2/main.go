package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/maneac/aoc2021/utils/lib/go/bench"
)

const (
	part1Solution = "1840243"
	part2Solution = "1727785422"
)

type Input []string

func main() {
	bench.Config{
		Filename:      "./bench/results/go/day_2.csv",
		DataDirectory: "./data",
		ReadData:      func(p string) bench.Day { return readData(p) },
		Part1Solution: part1Solution,
		Part2Solution: part2Solution,
	}.Run()
}

func readData(dir string) Input {
	contents, err := os.ReadFile(filepath.Join(dir, "day_2.txt"))
	if err != nil {
		panic(err)
	}

	return parseContents(strings.TrimSpace(string(contents)))
}

func parseContents(contents string) Input {
	return Input(strings.Split(contents, "\n"))
}

func (i Input) Part1() string {
	horizontal := 0
	depth := 0
	for _, line := range i {
		var instruction string
		var num int
		_, err := fmt.Sscan(line, &instruction, &num)
		if err != nil {
			panic(err)
		}

		switch instruction {
		case "forward":
			horizontal += num
		case "down":
			depth += num
		case "up":
			depth -= num
		default:
			panic("unknown instruction: " + instruction)
		}
	}
	return fmt.Sprint(horizontal * depth)
}

func (i Input) Part2() string {
	horizontal := 0
	depth := 0
	aim := 0
	for _, line := range i {
		var instruction string
		var num int
		_, err := fmt.Sscan(line, &instruction, &num)
		if err != nil {
			panic(err)
		}

		switch instruction {
		case "forward":
			horizontal += num
			depth += (aim * num)
		case "down":
			aim += num
		case "up":
			aim -= num
		default:
			panic("unknown instruction: " + instruction)
		}
	}
	return fmt.Sprint(horizontal * depth)
}
