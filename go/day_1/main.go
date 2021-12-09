package main

import (
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	data := readData()

	log.Println("Part 1: ", part1(data))
	log.Println("Part 2: ", part2(data))
}

func readData() []int {
	data, err := os.ReadFile("../../data/day_1.txt")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(strings.TrimSpace(string(data)), "\n")

	out := make([]int, len(lines))
	for idx, line := range lines {
		out[idx], err = strconv.Atoi(line)
		if err != nil {
			panic(err)
		}
	}
	return out
}

func part1(input []int) int {
	count := 0
	for idx, val := range input[1:] {
		if val > input[idx] {
			count++
		}
	}
	return count
}

func part2(input []int) int {
	count := 0
	for idx := range input[3:] {
		lastVal := input[idx] + input[idx+1] + input[idx+2]
		val := input[idx+1] + input[idx+2] + input[idx+3]
		if val > lastVal {
			count++
		}
	}
	return count
}
