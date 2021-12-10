package main

import (
	"log"
	"os"
	"sort"
	"strings"
)

func main() {
	data := readData()

	log.Println("Part 1: ", part1(data))
	log.Println("Part 2: ", part2(data))
}

func readData() []string {
	contents, err := os.ReadFile("../../data/day_10.txt")
	if err != nil {
		panic(err)
	}

	return parseContents(string(contents))
}

func parseContents(contents string) []string {
	return strings.Split(contents, "\n")
}

func part1(input []string) int {
	score := 0
	for _, line := range input {
		_, illegal := fixLine(line)
		if illegal == 0 {
			continue
		}
		switch illegal {
		case ')':
			score += 3
		case ']':
			score += 57
		case '}':
			score += 1197
		case '>':
			score += 25137
		}
	}
	return score
}

func part2(input []string) int {
	fixScores := make([]int, 0, len(input))
	for _, line := range input {
		fix, illegal := fixLine(line)
		if illegal != 0 {
			continue
		}

		fixScore := 0
		for _, chr := range fix {
			fixScore *= 5
			switch chr {
			case ')':
				fixScore += 1
			case ']':
				fixScore += 2
			case '}':
				fixScore += 3
			case '>':
				fixScore += 4
			}
		}

		fixScores = append(fixScores, fixScore)
	}

	sort.Ints(fixScores)
	return fixScores[len(fixScores)/2]
}

func fixLine(line string) (string, byte) {
	missing := ""
	for len(line) > 0 {
		chr := line[0]
		line = line[1:]

		switch chr {
		case '(':
			missing = ")" + missing
		case '[':
			missing = "]" + missing
		case '{':
			missing = "}" + missing
		case '<':
			missing = ">" + missing
		case ')', ']', '}', '>':
			match := missing[0]
			missing = missing[1:]
			if match != chr {
				return "", chr
			}
		}
	}
	return missing, 0
}
