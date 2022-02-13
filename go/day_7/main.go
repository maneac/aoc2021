package main

import (
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strconv"
	"strings"

	"github.com/maneac/aoc2021/utils/lib/go/bench"
)

const (
	part1Solution = "340056"
	part2Solution = "96592275"
)

type Input []int

func main() {
	bench.Config{
		Filename:      "./bench/results/go/day_7.csv",
		DataDirectory: "./data",
		ReadData:      func(p string) bench.Day { return readData(p) },
		Part1Solution: part1Solution,
		Part2Solution: part2Solution,
	}.Run()
}

func readData(dir string) Input {
	contents, err := os.ReadFile(filepath.Join(dir, "day_7.txt"))
	if err != nil {
		panic(err)
	}

	return parseContents(string(contents))
}

func parseContents(contents string) Input {
	nums := strings.Split(strings.TrimSpace(contents), ",")
	out := make(Input, len(nums))
	for i, num := range nums {
		n, err := strconv.Atoi(num)
		if err != nil {
			panic(err)
		}
		out[i] = n
	}
	sort.Ints(out)
	return out
}

func (i Input) Part1() string {
	target := i[len(i)/2]

	total := 0
	for _, crab := range i {
		if crab < target {
			total += target - crab
		} else {
			total += crab - target
		}
	}
	return fmt.Sprint(total)
}

func (i Input) Part2() string {
	total := -1
	for target := 0; target < i[len(i)-1]; target++ {
		subtotal := 0
		for _, crab := range i {
			diff := crab - target
			if crab < target {
				diff = target - crab
			}
			subtotal += (diff * (diff + 1)) / 2
		}
		if total < 0 || subtotal < total {
			total = subtotal
		}
	}
	return fmt.Sprint(total)
}
