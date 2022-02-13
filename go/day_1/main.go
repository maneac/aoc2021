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
	part1Solution = "1374"
	part2Solution = "1418"
)

type Input []int

func main() {
	bench.Config{
		Filename:      "./bench/results/go/day_1.csv",
		DataDirectory: "./data",
		ReadData:      func(p string) bench.Day { return readData(p) },
		Part1Solution: part1Solution,
		Part2Solution: part2Solution,
	}.Run()
}

func readData(dir string) Input {
	contents, err := os.ReadFile(filepath.Join(dir, "day_1.txt"))
	if err != nil {
		panic(err)
	}

	return parseContents(strings.TrimSpace(string(contents)))
}

func parseContents(contents string) Input {
	lines := strings.Split(contents, "\n")

	var err error
	out := make([]int, len(lines))
	for idx, line := range lines {
		out[idx], err = strconv.Atoi(line)
		if err != nil {
			panic(err)
		}
	}
	return out
}

func (i Input) Part1() string {
	count := 0
	for idx, val := range i[1:] {
		if val > i[idx] {
			count++
		}
	}
	return fmt.Sprint(count)
}

func (i Input) Part2() string {
	count := 0
	for idx := range i[3:] {
		lastVal := i[idx] + i[idx+1] + i[idx+2]
		val := i[idx+1] + i[idx+2] + i[idx+3]
		if val > lastVal {
			count++
		}
	}
	return fmt.Sprint(count)
}
