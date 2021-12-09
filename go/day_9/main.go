package main

import (
	"log"
	"os"
	"sort"
	"strings"
)

type input struct {
	heightmap map[point]int
	maxX      int
	maxY      int
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
	contents, err := os.ReadFile("../../data/day_9.txt")
	if err != nil {
		panic(err)
	}

	return parseContents(string(contents))
}

func parseContents(contents string) input {
	lines := strings.Split(strings.TrimSpace(contents), "\n")
	lineCount := len(lines)
	lineLengths := len(lines[0])

	out := make(map[point]int, lineCount*lineLengths)
	for y, line := range lines {
		for x, n := range strings.TrimSpace(line) {
			out[point{x: x, y: y}] = int(n - '0')
		}
	}

	return input{
		heightmap: out,
		maxX:      lineLengths - 1,
		maxY:      lineCount - 1,
	}
}

func part1(input input) int {
	totalRisk := 0
	for x := 0; x <= input.maxX; x++ {
		for y := 0; y <= input.maxY; y++ {
			totalRisk += input.calculateRisk(x, y)
		}
	}
	return totalRisk
}

func (i input) calculateRisk(targetX, targetY int) int {
	target, ok := i.heightmap[point{x: targetX, y: targetY}]
	if !ok {
		return 0
	}

	neighbours := []point{
		{x: targetX, y: targetY - 1}, // up
		{x: targetX - 1, y: targetY}, // left
		{x: targetX + 1, y: targetY}, // right
		{x: targetX, y: targetY + 1}, // down
	}

	for _, p := range neighbours {
		v, ok := i.heightmap[p]
		if !ok {
			continue
		}
		if v <= target {
			return 0
		}
	}

	return target + 1
}

func part2(input input) int {
	basinSizes := make([]int, len(input.heightmap)/2)
	for x := 0; x <= input.maxX; x++ {
		for y := 0; y <= input.maxY; y++ {
			if input.calculateRisk(x, y) > 0 {
				basinSizes = append(basinSizes, input.calculateBasinSize(x, y))
			}
		}
	}

	sort.Ints(basinSizes)

	return basinSizes[len(basinSizes)-3] * basinSizes[len(basinSizes)-2] * basinSizes[len(basinSizes)-1]
}

func (i input) calculateBasinSize(lowX, lowY int) int {
	var toSearch map[point]struct{}
	nextSearchList := map[point]struct{}{
		{x: lowX, y: lowY - 1}: {}, // up
		{x: lowX - 1, y: lowY}: {}, // left
		{x: lowX + 1, y: lowY}: {}, // right
		{x: lowX, y: lowY + 1}: {}, // down
	}

	basinContents := make(map[point]struct{}, len(i.heightmap)/2)
	basinContents[point{x: lowX, y: lowY}] = struct{}{}
	for len(nextSearchList) > 0 {
		toSearch = nextSearchList
		nextSearchList = make(map[point]struct{}, len(toSearch))

		for p := range toSearch {
			height, ok := i.heightmap[p]
			if !ok || height == 9 {
				continue
			}
			basinContents[p] = struct{}{}
			up := point{x: p.x, y: p.y - 1}
			if _, ok := basinContents[up]; !ok {
				nextSearchList[up] = struct{}{}
			}
			left := point{x: p.x - 1, y: p.y}
			if _, ok := basinContents[left]; !ok {
				nextSearchList[left] = struct{}{}
			}
			right := point{x: p.x + 1, y: p.y}
			if _, ok := basinContents[right]; !ok {
				nextSearchList[right] = struct{}{}
			}
			down := point{x: p.x, y: p.y + 1}
			if _, ok := basinContents[down]; !ok {
				nextSearchList[down] = struct{}{}
			}
		}
	}

	return len(basinContents)
}
