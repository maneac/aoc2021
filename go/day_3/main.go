package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/maneac/aoc2021/utils/lib/go/bench"
)

const (
	part1Solution = "3148794"
	part2Solution = "2795310"
)

type Input treeNode

func main() {
	bench.Config{
		Filename:      "./bench/results/go/day_3.csv",
		DataDirectory: "./data",
		ReadData:      func(p string) bench.Day { return readData(p) },
		Part1Solution: part1Solution,
		Part2Solution: part2Solution,
	}.Run()
}

func readData(dir string) Input {
	contents, err := os.ReadFile(filepath.Join(dir, "day_3.txt"))
	if err != nil {
		panic(err)
	}

	tree := parseContents(strings.TrimSpace(string(contents)))
	(*treeNode)(&tree).balance()
	return tree
}

func parseContents(contents string) Input {
	t := treeNode{}
	for _, line := range strings.Split(contents, "\n") {
		val := 0
		for idx, chr := range strings.TrimSpace(line) {
			if chr == '1' {
				val |= 1 << (len(line) - idx - 1)
			}
		}
		t.add(uint(val), len(line)-1)
	}
	return Input(t)
}

func (i Input) Part1() string {
	thisLayer := []*treeNode{i.children[0], i.children[1]}
	nextLayer := make([]*treeNode, 0, len(thisLayer)<<1)

	gammaRate := 0
	epsilonRate := 0
	for len(thisLayer) > 0 {
		onesTotal := 0
		zeroesTotal := 0
		for _, node := range thisLayer {
			if node == nil {
				continue
			}

			if node.childCount == 0 {
				switch node.value & 1 {
				case 0:
					zeroesTotal++
				case 1:
					onesTotal++
				}
				continue
			}

			switch node.value {
			case 0:
				zeroesTotal += node.childCount
			case 1:
				onesTotal += node.childCount
			}
			nextLayer = append(nextLayer, node.children[0], node.children[1])
		}

		thisLayer, nextLayer = nextLayer, make([]*treeNode, 0, len(nextLayer)<<1)
		gammaRate <<= 1
		epsilonRate <<= 1

		switch {
		case zeroesTotal > onesTotal:
			epsilonRate |= 1
		default:
			gammaRate |= 1
		}
	}

	return fmt.Sprint(gammaRate * epsilonRate)
}

func (i Input) Part2() string {
	oxygenRating := (*treeNode)(&i)
	for oxygenRating.childCount > 0 {
		if oxygenRating.children[1] != nil {
			oxygenRating = oxygenRating.children[1]
		} else {
			oxygenRating = oxygenRating.children[0]
		}
	}

	scrubberRating := (*treeNode)(&i)
	for scrubberRating.childCount > 0 {
		if scrubberRating.children[0] != nil {
			scrubberRating = scrubberRating.children[0]
		} else {
			scrubberRating = scrubberRating.children[1]
		}
	}

	return fmt.Sprint(oxygenRating.value * scrubberRating.value)
}

type treeNode struct {
	childCount int
	value      uint
	children   [2]*treeNode
}

func (t *treeNode) balance() {
	if t.childCount == 0 {
		return
	}

	if t.children[1] == nil {
		t.children[0].balance()
		if t.children[0].childCount > 0 {
			t.children[0], t.children[1] = t.children[1], t.children[0]
		}
		return
	}
	t.children[1].balance()

	if t.children[0] == nil {
		return
	}
	t.children[0].balance()

	if t.children[0].childCount > t.children[1].childCount {
		t.children[0], t.children[1] = t.children[1], t.children[0]
	}
}

func (t *treeNode) add(newVal uint, bitIdx int) {
	t.childCount++

	if bitIdx == 0 {
		// no duplicate entries, so children should be nil until initialised here
		t.children[newVal&1] = &treeNode{
			value: newVal,
		}
		return
	}

	childIdx := (newVal >> bitIdx) & 1
	if t.children[childIdx] == nil {
		t.children[childIdx] = &treeNode{
			value: childIdx,
		}
	}
	t.children[childIdx].add(newVal, bitIdx-1)
}

func (t *treeNode) String() string {
	if t == nil {
		return "\"\""
	}
	if t.childCount == 0 {
		return fmt.Sprint(t.value)
	}
	return fmt.Sprintf("{%s,%s}", t.children[0], t.children[1])
}
