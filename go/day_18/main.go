package main

import (
	"container/list"
	"fmt"
	"log"
	"os"
	"strings"
)

type input []*snailfishList

func main() {
	data := readData()

	log.Println("Part 1: ", part1(data))
	log.Println("Part 2: ", part2(data))
}

func readData() input {
	contents, err := os.ReadFile("../../data/day_18.txt")
	if err != nil {
		panic(err)
	}

	return parseContents(strings.TrimSpace(string(contents)))
}

func parseContents(contents string) input {
	lines := strings.Split(contents, "\n")

	out := make([]*snailfishList, 0, len(lines))
	for _, line := range lines {
		out = append(out, listFromLine(line))
	}

	return out
}

func listFromLine(line string) *snailfishList {
	out := newSnailfishList()
	level := -1
	val := -1
	for _, chr := range line {
		switch chr {
		case '[':
			level++
		case ']':
			if val >= 0 {
				out.PushBack(&snailfish{
					level: level,
					value: val,
				})
				val = -1
			}
			level--
		case '0', '1', '2', '3', '4', '5', '6', '7', '8', '9':
			if val < 0 {
				val = 0
			}
			val *= 10
			val += int(chr - '0')
		case ',':
			if val >= 0 {
				out.PushBack(&snailfish{
					level: level,
					value: val,
				})
				val = -1
			}
		default:
			panic(fmt.Sprintf("unexpected character: %c", chr))
		}
	}
	return out
}

func part1(input input) int {
	return input.reduce().magnitude()
}

func part2(in input) int {
	largest := 0
	for lidx, left := range in {
		for ridx, right := range in {
			if ridx == lidx {
				continue
			}
			if magnitude := part1(input{left.duplicate(), right.duplicate()}); magnitude > largest {
				largest = magnitude
			}
		}
	}
	return largest
}

func (i input) reduce() *snailfishList {
	out := i[0]
	if out == nil {
		return nil
	}
	out.reduce()

	for _, fish := range i[1:] {
		out.PushBackList(fish.List)
		for n := out.Front(); n != nil; n = n.Next() {
			n.Value.(*snailfish).level += 1
		}
		out.reduce()
	}
	return out
}

func (i input) String() string {
	var out strings.Builder
	out.WriteString("input[\n")

	for _, line := range i {
		out.WriteString("\t\t" + line.String() + ",\n")
	}
	out.WriteByte(']')
	return out.String()
}

type snailfishList struct {
	*list.List
}

func newSnailfishList() *snailfishList {
	return &snailfishList{
		List: list.New(),
	}
}

func (s *snailfishList) duplicate() *snailfishList {
	newList := &snailfishList{
		List: list.New(),
	}
	for n := s.Front(); n != nil; n = n.Next() {
		v := *n.Value.(*snailfish)
		newList.PushBack(&v)
	}
	return newList
}

func (s *snailfishList) magnitude() int {
	finalFish, _ := recursiveMagnitude(s.Front(), 0)
	return finalFish.value
}

func recursiveMagnitude(elem *list.Element, targetLevel int) (*snailfish, *list.Element) {
	// fmt.Printf("Lvl: %d: %v\n", targetLevel, elem.Value.(*snailfish))
	node := elem
	leftFish := node.Value.(*snailfish)
	if leftFish.level > targetLevel {
		leftFish, node = recursiveMagnitude(node, targetLevel+1)
		// fmt.Println("Left fish: ", leftFish)
	}

	node = node.Next()
	rightFish := node.Value.(*snailfish)
	if rightFish.level > targetLevel {
		rightFish, node = recursiveMagnitude(node, targetLevel+1)
		// fmt.Println("Right fish: ", leftFish)
	}

	return &snailfish{
		level: leftFish.level - 1,
		value: (3 * leftFish.value) + (2 * rightFish.value),
	}, node
}

func (s *snailfishList) reduce() {
	for s.reduceOnce() {
	}
}

func (s *snailfishList) reduceOnce() (modified bool) {
	for n := s.Front(); n != nil; n = n.Next() {
		f := n.Value.(*snailfish)
		if f.level == 4 {
			// fmt.Println(s)
			lhs := n
			rhs := n.Next()
			// if rhs == nil {
			// 	continue
			// }
			zero := &snailfish{
				level: 3,
				value: 0,
			}
			prev := lhs.Prev()
			next := rhs.Next()
			s.Remove(lhs)
			s.Remove(rhs)

			if prev != nil {
				prev.Value.(*snailfish).value += lhs.Value.(*snailfish).value
			}
			if next != nil {
				next.Value.(*snailfish).value += rhs.Value.(*snailfish).value
			}

			switch {
			case prev == nil:
				s.PushFront(zero)
			case next == nil:
				s.PushBack(zero)
			default:
				s.InsertAfter(zero, prev)
			}
			return true
		}
	}
	for n := s.Front(); n != nil; n = n.Next() {
		f := n.Value.(*snailfish)
		if f.value > 9 {
			v := f.value
			f.level++
			f.value = v / 2
			s.InsertAfter(&snailfish{
				level: f.level,
				value: v - f.value,
			}, n)
			return true
		}
	}
	return false
}

func (s *snailfishList) String() string {
	var out strings.Builder
	_, _ = out.WriteString("[ ")
	for n := s.Front(); n != nil; n = n.Next() {
		f := n.Value.(*snailfish)
		out.WriteString(fmt.Sprintf("{level: %d, value: %d} ", f.level, f.value))
	}
	_ = out.WriteByte(']')
	return out.String()
}

type snailfish struct {
	level int
	value int
}
