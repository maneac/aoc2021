package main

import (
	"bytes"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/maneac/aoc2021/utils/lib/go/bench"
)

const (
	part1Solution = "669"
	part2Solution = "uefzcucj"
)

type Input struct {
	points map[point]struct{}
	folds  []point
}

type point struct {
	x int
	y int
}

func main() {
	bench.Config{
		Filename:      "./bench/results/go/day_13.csv",
		DataDirectory: "./data",
		ReadData:      func(p string) bench.Day { return readData(p) },
		Part1Solution: part1Solution,
		Part2Solution: part2Solution,
	}.Run()
}

func readData(dir string) Input {
	contents, err := os.ReadFile(filepath.Join(dir, "day_13.txt"))
	if err != nil {
		panic(err)
	}

	return parseContents(string(contents))
}

func parseContents(contents string) Input {
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

	return Input{
		points: points,
		folds:  folds,
	}
}

func (i Input) Part1() string {
	fold := i.folds[0]
	for p := range i.points {
		if p.x < fold.x || p.y < fold.y {
			continue
		}
		delete(i.points, p)
		if fold.x > 0 {
			i.points[point{x: (2 * fold.x) - p.x, y: p.y}] = struct{}{}
			continue
		}
		i.points[point{x: p.x, y: (2 * fold.y) - p.y}] = struct{}{}
	}
	return fmt.Sprint(len(i.points))
}

func (i Input) Part2() string {
	for _, fold := range i.folds {
		for p := range i.points {
			if p.x < fold.x || p.y < fold.y {
				continue
			}
			delete(i.points, p)
			if fold.x > 0 {
				i.points[point{x: (2 * fold.x) - p.x, y: p.y}] = struct{}{}
				continue
			}
			i.points[point{x: p.x, y: (2 * fold.y) - p.y}] = struct{}{}
		}
	}

	max := point{x: 0, y: 0}
	for p := range i.points {
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
	for p := range i.points {
		display[p.x/5][p.y][p.x%5] = '#'
	}

	var out strings.Builder
	out.Grow(numChars)
	for _, char := range display {
		out.WriteByte(letters[string(bytes.Join(char, []byte{'\n'}))])
	}

	return out.String()
}
