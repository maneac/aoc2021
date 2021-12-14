package main

import (
	"log"
	"os"
	"strings"
)

type input struct {
	first      int
	last       int
	initial    map[int]int
	insertions map[int]int
}

func main() {
	data := readData()

	log.Println("Part 1: ", part1(data))
	log.Println("Part 2: ", part2(data))
}

func readData() input {
	contents, err := os.ReadFile("../../data/day_14.txt")
	if err != nil {
		panic(err)
	}

	return parseContents(string(contents))
}

func parseContents(contents string) input {
	parts := strings.Split(strings.TrimSpace(contents), "\n\n")

	initial := make(map[int]int, len(parts[0])-1)
	for i, c := range parts[0][1:] {
		initial[int(parts[0][i])<<8|int(c)] += 1
	}

	insertions := make(map[int]int, len(parts[1]))
	for _, line := range strings.Split(parts[1], "\n") {
		insertions[int(line[0])<<8|int(line[1])] = int(line[len(line)-1])
	}

	return input{
		first:      int(parts[0][0] - 'A'),
		last:       int(parts[0][len(parts[0])-1] - 'A'),
		initial:    initial,
		insertions: insertions,
	}
}

func part1(input input) int {
	return input.polymerise(10)
}

func part2(input input) int {
	return input.polymerise(40)
}

func (i input) polymerise(iterations int) int {
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
