package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/maneac/aoc2021/utils/lib/go/bench"
)

const (
	part1Solution = "1681"
	part2Solution = "276"
)

type Input [100]*octopus

type octopus struct {
	value      uint8
	neighbours []*octopus
}

func main() {
	bench.Config{
		Filename:      "./bench/results/go/day_11.csv",
		DataDirectory: "./data",
		ReadData:      func(p string) bench.Day { return readData(p) },
		Part1Solution: part1Solution,
		Part2Solution: part2Solution,
	}.Run()
}

func readData(dir string) Input {
	contents, err := os.ReadFile(filepath.Join(dir, "day_11.txt"))
	if err != nil {
		panic(err)
	}

	return parseContents(string(contents))
}

func parseContents(contents string) Input {
	octopodes := Input{}
	for idx := range octopodes {
		octopodes[idx] = new(octopus)
	}

	lines := strings.Split(strings.TrimSpace(contents), "\n")
	for y, row := range lines {
		for x, val := range row {
			idx := (10 * y) + x
			octopodes[idx].value = uint8(val - '0')
			for yn := y - 1; yn < y+2; yn++ {
				if yn < 0 || yn >= len(lines) {
					continue
				}
				for xn := x - 1; xn < x+2; xn++ {
					if xn < 0 || xn >= len(row) || (xn == x && yn == y) {
						continue
					}
					octopodes[idx].neighbours = append(octopodes[idx].neighbours, octopodes[(yn*10)+xn])
				}
			}
		}
	}

	return octopodes
}

func (i Input) Part1() string {
	flashCount := 0
	for iter := 0; iter < 100; iter++ {
		for _, octopus := range i {
			octopus.value++
		}

		for _, octopus := range i {
			if octopus.value > 9 {
				flashCount += i.flash(octopus)
			}
		}
	}
	return fmt.Sprint(flashCount)
}

func (i Input) Part2() string {
	iteration := 0
	for {
		iteration++
		for _, octopus := range i {
			octopus.value++
		}

		for _, octopus := range i {
			if octopus.value > 9 {
				i.flash(octopus)
			}
		}

		total := 0
		for _, octopus := range i {
			total += int(octopus.value)
		}
		if total == 0 {
			break
		}
	}
	return fmt.Sprint(iteration)
}

func (i Input) flash(octopus *octopus) int {
	if octopus.value < 9 {
		if octopus.value > 0 {
			octopus.value++
		}
		return 0
	}

	octopus.value = 0
	flashCount := 1
	for _, neighbour := range octopus.neighbours {
		flashCount += i.flash(neighbour)
	}
	return flashCount
}
