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
	part1Solution = "360761"
	part2Solution = "1632779838045"
)

type Input [9]int

func main() {
	bench.Config{
		Filename:      "./bench/results/go/day_6.csv",
		DataDirectory: "./data",
		ReadData:      func(p string) bench.Day { return readData(p) },
		Part1Solution: part1Solution,
		Part2Solution: part2Solution,
	}.Run()
}

func readData(dir string) Input {
	contents, err := os.ReadFile(filepath.Join(dir, "day_6.txt"))
	if err != nil {
		panic(err)
	}
	return parseContents(string(contents))
}

func parseContents(contents string) Input {
	out := Input{}
	for _, num := range strings.Split(strings.TrimSpace(contents), ",") {
		idx, err := strconv.Atoi(num)
		if err != nil {
			panic(err)
		}
		out[idx]++
	}
	return out
}

func (i Input) Part1() string {
	return fmt.Sprint(i.fishAfterDays(80))
}

func (i Input) Part2() string {
	return fmt.Sprint(i.fishAfterDays(256))
}

func (i Input) fishAfterDays(days int) int {
	fish := i[:]
	for i := 0; i < days; i++ {
		fish = append(fish[1:], fish[0])
		fish[6] += fish[8]
	}

	total := 0
	for _, count := range fish {
		total += count
	}
	return total
}
