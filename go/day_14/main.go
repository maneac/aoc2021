package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/maneac/aoc2021/utils/lib/go/bench"
)

const (
	part1Solution = "2584"
	part2Solution = "3816397135460"
)

type Input struct {
	first      int
	last       int
	initial    map[int]int
	insertions map[int]int
}

func main() {
	bench.Config{
		Filename:      "./bench/results/go/day_14.csv",
		DataDirectory: "./data",
		ReadData:      func(p string) bench.Day { return readData(p) },
		Part1Solution: part1Solution,
		Part2Solution: part2Solution,
	}.Run()
}

func readData(dir string) Input {
	contents, err := os.ReadFile(filepath.Join(dir, "day_14.txt"))
	if err != nil {
		panic(err)
	}

	return parseContents(string(contents))
}

func parseContents(contents string) Input {
	parts := strings.Split(strings.TrimSpace(contents), "\n\n")

	initial := make(map[int]int, len(parts[0])-1)
	for i, c := range parts[0][1:] {
		initial[int(parts[0][i])<<8|int(c)] += 1
	}

	insertions := make(map[int]int, len(parts[1]))
	for _, line := range strings.Split(parts[1], "\n") {
		insertions[int(line[0])<<8|int(line[1])] = int(line[len(line)-1])
	}

	return Input{
		first:      int(parts[0][0] - 'A'),
		last:       int(parts[0][len(parts[0])-1] - 'A'),
		initial:    initial,
		insertions: insertions,
	}
}

func (i Input) Part1() string {
	return fmt.Sprint(i.polymerise(10))
}

func (i Input) Part2() string {
	return fmt.Sprint(i.polymerise(40))
}

func (i Input) polymerise(iterations int) int {
	polymer := i.initial
	newPolymer := make(map[int]int, len(i.initial))

	for iter := 0; iter < iterations; iter++ {
		for k, oldCount := range polymer {
			if chr, ok := i.insertions[k]; ok {
				newPolymer[(k&0xFF00)|chr] += oldCount
				newPolymer[(chr<<8)|(k&0xFF)] += oldCount
				continue
			}
			newPolymer[k] += oldCount
		}
		polymer, newPolymer = newPolymer, polymer
		for k := range newPolymer {
			delete(newPolymer, k)
		}
	}

	counts := make([]int, 26)
	for k, v := range polymer {
		counts[(k>>8)-'A'] += v
		counts[(k&0xFF)-'A'] += v
	}

	max := -1
	min := -1
	for idx, rawCount := range counts {
		if rawCount == 0 {
			continue
		}
		count := rawCount
		if idx == i.first {
			count++
		}
		if idx == i.last {
			count++
		}
		count /= 2
		if max < 0 || count > max {
			max = count
		}
		if min < 0 || count < min {
			min = count
		}
	}

	return max - min
}
