package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

type vectors [][2]point

type point struct {
	x int
	y int
}

type points map[point]int

func main() {
	data := readData()

	log.Println("Part 1: ", part1(data))
	log.Println("Part 2: ", part2(data))
}

func readData() vectors {
	contents, err := os.ReadFile("../../data/day_5.txt")
	if err != nil {
		panic(err)
	}

	return parseContents(string(contents))
}

func parseContents(contents string) vectors {
	lines := strings.Split(strings.TrimSpace(contents), "\n")

	output := make(vectors, len(lines))
	for idx, line := range lines {
		var from, to point
		_, err := fmt.Sscanf(line, "%d,%d -> %d,%d", &from.x, &from.y, &to.x, &to.y)
		if err != nil {
			panic(err)
		}
		output[idx] = [2]point{from, to}
	}
	return output
}

func part1(input vectors) int {
	points := points{}

	for _, v := range input {
		minX := v[0].x
		maxX := v[1].x
		if minX > maxX {
			minX, maxX = maxX, minX
		}

		minY := v[0].y
		maxY := v[1].y
		if minY > maxY {
			minY, maxY = maxY, minY
		}

		if minX != maxX && minY != maxY {
			continue
		}
		for x := minX; x <= maxX; x++ {
			for y := minY; y <= maxY; y++ {
				points[point{x: x, y: y}]++
			}
		}
	}

	overlaps := 0
	for _, count := range points {
		if count > 1 {
			overlaps++
		}
	}

	return overlaps
}

func part2(input vectors) int {
	points := points{}

	for _, v := range input {
		minX := v[0].x
		maxX := v[1].x
		xDiff := maxX - minX
		if minX > maxX {
			minX, maxX = maxX, minX
		}

		minY := v[0].y
		maxY := v[1].y
		yDiff := maxY - minY
		if minY > maxY {
			minY, maxY = maxY, minY
		}

		if (maxX - minX) == (maxY - minY) {
			switch {
			case xDiff > 0 && yDiff > 0:
				for i := 0; i <= (maxX - minX); i++ {
					points[point{x: minX + i, y: minY + i}]++
				}
			case xDiff > 0:
				for i := 0; i <= (maxX - minX); i++ {
					points[point{x: minX + i, y: maxY - i}]++
				}
			case yDiff > 0:
				for i := 0; i <= (maxX - minX); i++ {
					points[point{x: maxX - i, y: minY + i}]++
				}
			default:
				for i := 0; i <= (maxX - minX); i++ {
					points[point{x: maxX - i, y: maxY - i}]++
				}
			}
			continue
		}

		for x := minX; x <= maxX; x++ {
			for y := minY; y <= maxY; y++ {
				points[point{x: x, y: y}]++
			}
		}
	}

	overlaps := 0
	for _, count := range points {
		if count > 1 {
			overlaps++
		}
	}

	return overlaps
}

func (points points) String() string {
	maxX := 0
	maxY := 0
	for p := range points {
		if p.x > maxX {
			maxX = p.x
		}
		if p.y > maxY {
			maxY = p.y
		}
	}

	var out strings.Builder
	out.Grow(maxY * (1 + maxX))
	for y := 0; y <= maxY; y++ {
		for x := 0; x <= maxX; x++ {
			count := points[point{x: x, y: y}]
			if count == 0 {
				out.WriteByte('.')
				continue
			}
			out.WriteString(fmt.Sprint(count))
		}
		out.WriteByte('\n')
	}
	return out.String()
}
