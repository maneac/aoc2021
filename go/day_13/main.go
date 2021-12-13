package main

import (
	"bytes"
	"fmt"
	"log"
	"os"
	"strings"
)

type input struct {
	points map[point]struct{}
	folds  []point
}

type point struct {
	x int
	y int
}

func main() {
	data := readData()

	log.Println("Part 1: ", part1(data))
	log.Println("Part 2: ", part2(data))
}

func readData() input {
	contents, err := os.ReadFile("../../data/day_13.txt")
	if err != nil {
		panic(err)
	}

	return parseContents(string(contents))
}

func parseContents(contents string) input {
	sections := strings.SplitN(strings.TrimSpace(contents), "\n\n", 2)

	pointLines := strings.Split(sections[0], "\n")

	points := make(map[point]struct{}, len(pointLines))
	for _, line := range pointLines {
		var p point
		_, err := fmt.Sscanf(line, "%d,%d", &p.x, &p.y)
		if err != nil {
			panic(err)
		}
		points[p] = struct{}{}
	}

	foldLines := strings.Split(sections[1], "\n")
	folds := make([]point, 0, len(foldLines))
	for _, line := range foldLines {
		var axis byte
		var coordinate int
		_, err := fmt.Sscanf(line, "fold along %c=%d", &axis, &coordinate)
		if err != nil {
			panic(err)
		}
		switch axis {
		case 'x':
			folds = append(folds, point{x: coordinate, y: 0})
		case 'y':
			folds = append(folds, point{x: 0, y: coordinate})
		default:
			panic(fmt.Sprintf("unknown axis: %c", axis))
		}
	}

	return input{
		points: points,
		folds:  folds,
	}
}

func part1(input input) int {
	fold := input.folds[0]
	for p := range input.points {
		if p.x < fold.x || p.y < fold.y {
			continue
		}
		delete(input.points, p)
		if fold.x > 0 {
			input.points[point{x: (2 * fold.x) - p.x, y: p.y}] = struct{}{}
			continue
		}
		input.points[point{x: p.x, y: (2 * fold.y) - p.y}] = struct{}{}
	}
	return len(input.points)
}

func part2(input input) string {
	for _, fold := range input.folds {
		for p := range input.points {
			if p.x < fold.x || p.y < fold.y {
				continue
			}
			delete(input.points, p)
			if fold.x > 0 {
				input.points[point{x: (2 * fold.x) - p.x, y: p.y}] = struct{}{}
				continue
			}
			input.points[point{x: p.x, y: (2 * fold.y) - p.y}] = struct{}{}
		}
	}

	max := point{x: 0, y: 0}
	for p := range input.points {
		if p.x > max.x {
			max.x = p.x
		}
		if p.y > max.y {
			max.y = p.y
		}
	}

	numChars := (max.x + 4) / 5

	display := make([][][]byte, numChars)
	for idx := range display {
		char := make([][]byte, max.y+1)
		for c := range char {
			char[c] = []byte{'.', '.', '.', '.', '.'}
		}
		display[idx] = char
	}
	for p := range input.points {
		display[p.x/5][p.y][p.x%5] = '#'
	}

	var out strings.Builder
	out.Grow(numChars)
	for _, char := range display {
		out.WriteByte(letters[string(bytes.Join(char, []byte{'\n'}))])
	}

	return out.String()
}
