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

func readData() *treeNode {
	contents, err := os.ReadFile("../../data/day_3.txt")
	if err != nil {
		panic(err)
	}

	tree := parseContents(string(contents))
	tree.balance()
	return tree
}

func part1(input *treeNode) string {
	thisLayer := []*treeNode{input.children[0], input.children[1]}
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

func part2(input *treeNode) string {
	oxygenRating := input
	for oxygenRating.childCount > 0 {
		if oxygenRating.children[1] != nil {
			oxygenRating = oxygenRating.children[1]
		} else {
			oxygenRating = oxygenRating.children[0]
		}
	}

	scrubberRating := input
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

func parseContents(contents string) *treeNode {
	t := &treeNode{}
	for _, line := range strings.Split(strings.TrimSpace(contents), "\n") {
		val := 0
		for idx, chr := range strings.TrimSpace(line) {
			if chr == '1' {
				val |= 1 << (len(line) - idx - 1)
			}
		}
		t.add(uint(val), len(line)-1)
	}
	return t
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
