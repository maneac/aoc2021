package main

import (
	"log"
	"os"
	"strings"
)

type input [100]*octopus

type octopus struct {
	value      uint8
	neighbours []*octopus
}

func main() {
	data := readData()

	log.Println("Part 1: ", part1(data))
	log.Println("Part 2: ", part2(data))
}

func readData() input {
	contents, err := os.ReadFile("../../data/day_11.txt")
	if err != nil {
		panic(err)
	}

	return parseContents(string(contents))
}

func parseContents(contents string) input {
	octopodes := input{}
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

func part1(input input) int {
	flashCount := 0
	for i := 0; i < 100; i++ {
		for _, octopus := range input {
			octopus.value++
		}

		for _, octopus := range input {
			if octopus.value > 9 {
				flashCount += input.flash(octopus)
			}
		}
	}
	return flashCount
}

func part2(input input) int {
	iteration := 0
	for {
		iteration++
		for _, octopus := range input {
			octopus.value++
		}

		for _, octopus := range input {
			if octopus.value > 9 {
				input.flash(octopus)
			}
		}

		total := 0
		for _, octopus := range input {
			total += int(octopus.value)
		}
		if total == 0 {
			break
		}
	}
	return iteration
}

func (i input) flash(octopus *octopus) int {
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
