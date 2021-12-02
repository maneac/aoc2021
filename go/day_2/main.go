package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	data := readData()

	log.Println("Part 1: ", part1(data))
	log.Println("Part 2: ", part2(data))
}

func readData() []string {
	contents, err := os.ReadFile("../../data/day_2.txt")
	if err != nil {
		panic(err)
	}
	return strings.Split(strings.TrimSpace(string(contents)), "\n")
}

func part1(input []string) string {
	horizontal := 0
	depth := 0
	for _, line := range input {
		var instruction string
		var num int
		_, err := fmt.Sscan(line, &instruction, &num)
		if err != nil {
			panic(err)
		}

		switch instruction {
		case "forward":
			horizontal += num
		case "down":
			depth += num
		case "up":
			depth -= num
		default:
			panic("unknown instruction: " + instruction)
		}
	}
	return fmt.Sprint(horizontal * depth)
}

func part2(input []string) string {
	horizontal := 0
	depth := 0
	aim := 0
	for _, line := range input {
		var instruction string
		var num int
		_, err := fmt.Sscan(line, &instruction, &num)
		if err != nil {
			panic(err)
		}

		switch instruction {
		case "forward":
			horizontal += num
			depth += (aim * num)
		case "down":
			aim += num
		case "up":
			aim -= num
		default:
			panic("unknown instruction: " + instruction)
		}
	}
	return fmt.Sprint(horizontal * depth)
}
