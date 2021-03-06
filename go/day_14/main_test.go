package main

import (
	"reflect"
	"testing"
)

func TestParseContents(t *testing.T) {
	tests := map[string]struct {
		data     string
		expected Input
	}{
		"actual": {
			data: `NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C`,
			expected: exampleData(),
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

func TestPart1(t *testing.T) {
	tests := map[string]struct {
		data     Input
		expected string
	}{
		"example": {
			data:     exampleData(),
			expected: "1588",
		},
		"actual": {
			data:     readData("../../data"),
			expected: part1Solution,
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			actual := test.data.Part1()

			if actual != test.expected {
				t.Fatalf("Expected: %v\nActual: %v", test.expected, actual)
			}
		})
	}
}

func TestPart2(t *testing.T) {
	tests := map[string]struct {
		data     Input
		expected string
	}{
		"example": {
			data:     exampleData(),
			expected: "2188189693529",
		},
		"actual": {
			data:     readData("../../data"),
			expected: part2Solution,
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			actual := test.data.Part2()

			if actual != test.expected {
				t.Fatalf("Expected: %v\nActual: %v", test.expected, actual)
			}
		})
	}
}

func exampleData() Input {
	return Input{
		first: 'N' - 'A',
		last:  'B' - 'A',
		initial: map[int]int{
			'N'<<8 | 'N': 1,
			'N'<<8 | 'C': 1,
			'C'<<8 | 'B': 1,
		},
		insertions: map[int]int{
			'C'<<8 | 'H': 'B',
			'H'<<8 | 'H': 'N',
			'C'<<8 | 'B': 'H',
			'N'<<8 | 'H': 'C',
			'H'<<8 | 'B': 'C',
			'H'<<8 | 'C': 'B',
			'H'<<8 | 'N': 'C',
			'N'<<8 | 'N': 'C',
			'B'<<8 | 'H': 'H',
			'N'<<8 | 'C': 'B',
			'N'<<8 | 'B': 'B',
			'B'<<8 | 'N': 'B',
			'B'<<8 | 'B': 'N',
			'B'<<8 | 'C': 'B',
			'C'<<8 | 'C': 'N',
			'C'<<8 | 'N': 'C',
		},
	}
}

func BenchmarkReadData(b *testing.B) {
	for i := 0; i < b.N; i++ {
		if readData("../../data").insertions == nil {
			b.FailNow()
		}
	}
}

func BenchmarkPart1(b *testing.B) {
	for i := 0; i < b.N; i++ {
		b.StopTimer()
		data := readData("../../data")
		b.StartTimer()
		if data.Part1() != part1Solution {
			b.FailNow()
		}
	}
}

func BenchmarkPart2(b *testing.B) {
	for i := 0; i < b.N; i++ {
		b.StopTimer()
		data := readData("../../data")
		b.StartTimer()
		if data.Part2() != part2Solution {
			b.FailNow()
		}
	}
}
