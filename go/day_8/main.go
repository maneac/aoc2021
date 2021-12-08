package main

import (
	"log"
	"math/bits"
	"os"
	"strings"
)

var nums = map[uint8]int{
	0b1110111: 0,
	0b0100100: 1,
	0b1011101: 2,
	0b1101101: 3,
	0b0101110: 4,
	0b1101011: 5,
	0b1111011: 6,
	0b0100101: 7,
	0b1111111: 8,
	0b1101111: 9,
}

type display struct {
	digits [10]uint8
	value  [4]uint8
}

func main() {
	data := readData()

	log.Println("Part 1: ", part1(data))
	log.Println("Part 2: ", part2(data))
}

func readData() []display {
	contents, err := os.ReadFile("../../data/day_8.txt")
	if err != nil {
		panic(err)
	}

	return parseContents(string(contents))
}

func parseContents(contents string) []display {
	lines := strings.Split(strings.TrimSpace(contents), "\n")
	displays := make([]display, 0, len(lines))

	for _, line := range lines {
		lineParts := strings.Split(line, " | ")

		digits := [10]uint8{}
		value := [4]uint8{}

		for idx, digit := range strings.Split(lineParts[0], " ") {
			num := uint8(0)
			for _, chr := range digit {
				num |= (1 << (chr - 'a'))
			}
			digits[idx] = num
		}

		for idx, digit := range strings.Split(lineParts[1], " ") {
			num := uint8(0)
			for _, chr := range digit {
				num |= (1 << (chr - 'a'))
			}
			value[idx] = num
		}

		displays = append(displays, display{digits: digits, value: value})
	}

	return displays
}

func part1(input []display) int {
	total := 0
	for _, display := range input {
		for _, v := range display.value {
			switch bits.OnesCount8(v) {
			case 2, 3, 4, 7:
				total++
			}
		}
	}
	return total
}

func part2(input []display) int {
	total := 0
	for _, display := range input {
		one := find(display.digits[:], func(u uint8) bool { return bits.OnesCount8(u) == 2 })
		seven := find(display.digits[:], func(u uint8) bool { return bits.OnesCount8(u) == 3 })
		four := find(display.digits[:], func(u uint8) bool { return bits.OnesCount8(u) == 4 })

		// &^: bitwise difference
		// 1: __c__f_
		// 7: a_c__f_

		// a = 7 - 1
		a := seven &^ one

		// 5s:
		// 2: a_cde_g
		// 3: a_cd_fg
		// 5: ab_d_fg
		lenFives := findAll(display.digits[:], func(u uint8) bool { return bits.OnesCount8(u) == 5 })

		// 3 contains 1 => 3 identified
		three := find(lenFives, func(u uint8) bool { return u&one == one })

		// 3: a_cd_fg
		// 4: _bcd_f_
		// (4 n 3): __cd_f_

		// b = 4 - (4 n 3)
		b := four &^ (four & three)

		// g = 3 - (4 n 3) - a
		g := three &^ (four & three) &^ a

		// 5 contains b => 5 and 2 identified
		five := find(lenFives, func(u uint8) bool { return u&b > 0 })
		two := find(lenFives, func(u uint8) bool { return u != five && u != three })

		// (3 n 5): a__d_fg
		// (2 n 3): a_cd__g

		// c = 3 - (3 n 5)
		c := three &^ (three & five)

		// e = 2 - (2 n 3)
		e := two &^ (two & three)

		// f = 3 - (2 n 3)
		f := three &^ (two & three)

		// d = remainder
		d := uint8(0b1111111) &^ a &^ b &^ c &^ e &^ f &^ g

		mapping := map[uint8]uint8{
			a: 0b0000001,
			b: 0b0000010,
			c: 0b0000100,
			d: 0b0001000,
			e: 0b0010000,
			f: 0b0100000,
			g: 0b1000000,
		}

		for idx, v := range display.value {
			asBits := uint8(0)
			for k, b := range mapping {
				if v&k > 0 {
					asBits |= b
				}
			}
			n := nums[asBits]
			for i := 0; i < (4 - idx - 1); i++ {
				n *= 10
			}
			total += n
		}
	}
	return total
}

func find(in []uint8, filter func(uint8) bool) uint8 {
	for _, v := range in {
		if filter(v) {
			return v
		}
	}
	panic("no match")
}

func findAll(in []uint8, filter func(uint8) bool) []uint8 {
	out := make([]uint8, 0, len(in))
	for _, v := range in {
		if filter(v) {
			out = append(out, v)
		}
	}
	return out
}
