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

func readData() [9]int {
	contents, err := os.ReadFile("../../data/day_6.txt")
	if err != nil {
		panic(err)
	}
	return parseContents(string(contents))
}

func parseContents(contents string) [9]int {
	out := [9]int{}
	for _, num := range strings.Split(strings.TrimSpace(contents), ",") {
		idx, err := strconv.Atoi(num)
		if err != nil {
			panic(err)
		}
		out[idx]++
	}
	return out
}

func part1(input [9]int) int {
	return fishAfterDays(input, 80)
}

func part2(input [9]int) int {
	return fishAfterDays(input, 256)
}

func fishAfterDays(input [9]int, days int) int {
	fish := input[:]
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
