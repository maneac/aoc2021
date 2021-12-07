package main

import (
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	data := readData()

	log.Println("Part 1: ", part1(data))
	log.Println("Part 2: ", part2(data))
}

func readData() []int {
	contents, err := os.ReadFile("../../data/day_7.txt")
	if err != nil {
		panic(err)
	}

	return parseContents(string(contents))
}

func parseContents(contents string) []int {
	nums := strings.Split(strings.TrimSpace(contents), ",")
	out := make([]int, len(nums))
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

func part1(input []int) int {
	target := input[len(input)/2]

	total := 0
	for _, crab := range input {
		if crab < target {
			total += target - crab
		} else {
			total += crab - target
		}
	}
	return total
}

func part2(input []int) int {
	total := -1
	for target := 0; target < input[len(input)-1]; target++ {
		subtotal := 0
		for _, crab := range input {
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
	return total
}
