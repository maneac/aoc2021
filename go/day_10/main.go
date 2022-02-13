package main

import (
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strings"

	"github.com/maneac/aoc2021/utils/lib/go/bench"
)

const (
	part1Solution = "266301"
	part2Solution = "3404870164"
)

type Input []string

func main() {
	bench.Config{
		Filename:      "./bench/results/go/day_10.csv",
		DataDirectory: "./data",
		ReadData:      func(p string) bench.Day { return readData(p) },
		Part1Solution: part1Solution,
		Part2Solution: part2Solution,
	}.Run()
}

func readData(dir string) Input {
	contents, err := os.ReadFile(filepath.Join(dir, "day_10.txt"))
	if err != nil {
		panic(err)
	}

	return parseContents(string(contents))
}

func parseContents(contents string) Input {
	return strings.Split(contents, "\n")
}

func (i Input) Part1() string {
	score := 0
	for _, line := range i {
		_, illegal := fixLine(line)
		if illegal == 0 {
			continue
		}
		switch illegal {
		case ')':
			score += 3
		case ']':
			score += 57
		case '}':
			score += 1197
		case '>':
			score += 25137
		}
	}
	return fmt.Sprint(score)
}

func (i Input) Part2() string {
	fixScores := make([]int, 0, len(i))
	for _, line := range i {
		fix, illegal := fixLine(line)
		if illegal != 0 {
			continue
		}

		fixScore := 0
		for _, chr := range fix {
			fixScore *= 5
			switch chr {
			case ')':
				fixScore += 1
			case ']':
				fixScore += 2
			case '}':
				fixScore += 3
			case '>':
				fixScore += 4
			}
		}

		fixScores = append(fixScores, fixScore)
	}

	sort.Ints(fixScores)
	return fmt.Sprint(fixScores[len(fixScores)/2])
}

func fixLine(line string) (string, byte) {
	missing := ""
	for len(line) > 0 {
		chr := line[0]
		line = line[1:]

		switch chr {
		case '(':
			missing = ")" + missing
		case '[':
			missing = "]" + missing
		case '{':
			missing = "}" + missing
		case '<':
			missing = ">" + missing
		case ')', ']', '}', '>':
			match := missing[0]
			missing = missing[1:]
			if match != chr {
				return "", chr
			}
		}
	}
	return missing, 0
}
