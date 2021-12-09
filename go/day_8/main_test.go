package main

import (
	"reflect"
	"testing"
)

const (
	part1Solution = 284
	part2Solution = 973499
)

func TestPart1(t *testing.T) {
	tests := map[string]struct {
		data     []display
		expected int
	}{
		"example1": {
			data:     example1Data(),
			expected: 0,
		},
		"example2": {
			data:     example2Data(),
			expected: 26,
		},
		"actual": {
			data:     readData(),
			expected: part1Solution,
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			actual := part1(test.data)

			if actual != test.expected {
				t.Fatalf("Expected: %v\nActual: %v", test.expected, actual)
			}
		})
	}
}

func TestPart2(t *testing.T) {
	tests := map[string]struct {
		data     []display
		expected int
	}{
		"example1": {
			data:     example1Data(),
			expected: 5353,
		},
		"example2": {
			data:     example2Data(),
			expected: 61229,
		},
		"actual": {
			data:     readData(),
			expected: part2Solution,
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			actual := part2(test.data)

			if actual != test.expected {
				t.Fatalf("Expected: %v\nActual: %v", test.expected, actual)
			}
		})
	}
}

func TestParseContents(t *testing.T) {
	tests := map[string]struct {
		data     string
		expected []display
	}{
		"example1": {
			data:     "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
			expected: example1Data(),
		},
		"example2": {
			data: `be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce`,
			expected: example2Data(),
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			actual := parseContents(test.data)

			if !reflect.DeepEqual(actual, test.expected) {
				t.Fatalf("Expected: %v\nActual: %v", test.expected, actual)
			}
		})
	}
}

func example1Data() []display {
	return []display{
		{
			digits: [10]uint8{
				0b1111111,
				0b0111110,
				0b1101101,
				0b0101111,
				0b0001011,
				0b0111111,
				0b1111110,
				0b0110011,
				0b1011111,
				0b0000011,
			},
			value: [4]uint8{
				0b0111110,
				0b0101111,
				0b0111110,
				0b0101111,
			},
		},
	}
}

func example2Data() []display {
	return []display{
		{
			digits: [10]uint8{
				0b0010010,
				0b1111111,
				0b1111110,
				0b1111101,
				0b1010110,
				0b1111100,
				0b1111011,
				0b0111110,
				0b0101111,
				0b0011010,
			},
			value: [4]uint8{
				0b1111111,
				0b0111110,
				0b1111110,
				0b1010110,
			},
		},
		{
			digits: [10]uint8{
				0b1111011,
				0b1011110,
				0b1000110,
				0b1000100,
				0b1111111,
				0b1111010,
				0b1101111,
				0b0011111,
				0b1111110,
				0b1110100,
			},
			value: [4]uint8{
				0b1111110,
				0b1000110,
				0b1111111,
				0b1000100,
			},
		},
		{
			digits: [10]uint8{
				0b1111011,
				0b1000100,
				0b0011111,
				0b1101011,
				0b1101111,
				0b1111110,
				0b1001111,
				0b1100101,
				0b1000110,
				0b1111111,
			},
			value: [4]uint8{
				0b1000100,
				0b1000100,
				0b1101111,
				0b1000110,
			},
		},
		{
			digits: [10]uint8{
				0b1111110,
				0b0001110,
				0b0111111,
				0b1011011,
				0b0100111,
				0b0000110,
				0b0111101,
				0b0011111,
				0b1111101,
				0b1111111,
			},
			value: [4]uint8{
				0b0111111,
				0b0011111,
				0b1111101,
				0b0000110,
			},
		},
		{
			digits: [10]uint8{
				0b1111111,
				0b1100010,
				0b1100000,
				0b1110011,
				0b0111011,
				0b1110100,
				0b1010111,
				0b1110111,
				0b1011111,
				0b1101111,
			},
			value: [4]uint8{
				0b1110100,
				0b1111111,
				0b1100010,
				0b1110011,
			},
		},
		{
			digits: [10]uint8{
				0b1110011,
				0b0000101,
				0b1110111,
				0b1111111,
				0b1111101,
				0b1101110,
				0b0010111,
				0b1111011,
				0b1100111,
				0b0100101,
			},
			value: [4]uint8{
				0b1111111,
				0b0010111,
				0b0000101,
				0b1111111,
			},
		},
		{
			digits: [10]uint8{
				0b1101110,
				0b1101000,
				0b1111111,
				0b1110100,
				0b1111011,
				0b0111111,
				0b0111110,
				0b1001111,
				0b1111110,
				0b1100000,
			},
			value: [4]uint8{
				0b1110100,
				0b0111110,
				0b1110100,
				0b1111111,
			},
		},
		{
			digits: [10]uint8{
				0b1111110,
				0b1110111,
				0b1110110,
				0b1111101,
				0b1001111,
				0b0011000,
				0b0111010,
				0b0011100,
				0b1111111,
				0b1011110,
			},
			value: [4]uint8{
				0b0011000,
				0b1110111,
				0b1001111,
				0b1110110,
			},
		},
		{
			digits: [10]uint8{
				0b1111011,
				0b1111110,
				0b1011100,
				0b0110111,
				0b1000110,
				0b1111111,
				0b1000100,
				0b1101111,
				0b1111010,
				0b1110110,
			},
			value: [4]uint8{
				0b1111111,
				0b1000110,
				0b1000100,
				0b1000110,
			},
		},
		{
			digits: [10]uint8{
				0b1100111,
				0b1100100,
				0b1111111,
				0b1010111,
				0b1100000,
				0b1011111,
				0b1110001,
				0b1110111,
				0b0101111,
				0b1111110,
			},
			value: [4]uint8{
				0b1110001,
				0b1100111,
				0b1100000,
				0b1010111,
			},
		},
	}
}

func BenchmarkReadData(b *testing.B) {
	for i := 0; i < b.N; i++ {
		if readData() == nil {
			b.FailNow()
		}
	}
}

func BenchmarkPart1(b *testing.B) {
	data := readData()
	for i := 0; i < b.N; i++ {
		if part1(data) != part1Solution {
			b.FailNow()
		}
	}
}

func BenchmarkPart2(b *testing.B) {
	data := readData()
	for i := 0; i < b.N; i++ {
		if part2(data) != part2Solution {
			b.FailNow()
		}
	}
}
