package main

import (
	"reflect"
	"testing"
)

const (
	part1Solution = 5608
	part2Solution = 20299
)

func TestPart1(t *testing.T) {
	tests := map[string]struct {
		data     vectors
		expected int
	}{
		"example": {
			data: vectors{
				[2]point{{x: 0, y: 9}, {x: 5, y: 9}},
				[2]point{{x: 8, y: 0}, {x: 0, y: 8}},
				[2]point{{x: 9, y: 4}, {x: 3, y: 4}},
				[2]point{{x: 2, y: 2}, {x: 2, y: 1}},
				[2]point{{x: 7, y: 0}, {x: 7, y: 4}},
				[2]point{{x: 6, y: 4}, {x: 2, y: 0}},
				[2]point{{x: 0, y: 9}, {x: 2, y: 9}},
				[2]point{{x: 3, y: 4}, {x: 1, y: 4}},
				[2]point{{x: 0, y: 0}, {x: 8, y: 8}},
				[2]point{{x: 5, y: 5}, {x: 8, y: 2}},
			},
			expected: 5,
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
		data     vectors
		expected int
	}{
		"example": {
			data: vectors{
				[2]point{{x: 0, y: 9}, {x: 5, y: 9}},
				[2]point{{x: 8, y: 0}, {x: 0, y: 8}},
				[2]point{{x: 9, y: 4}, {x: 3, y: 4}},
				[2]point{{x: 2, y: 2}, {x: 2, y: 1}},
				[2]point{{x: 7, y: 0}, {x: 7, y: 4}},
				[2]point{{x: 6, y: 4}, {x: 2, y: 0}},
				[2]point{{x: 0, y: 9}, {x: 2, y: 9}},
				[2]point{{x: 3, y: 4}, {x: 1, y: 4}},
				[2]point{{x: 0, y: 0}, {x: 8, y: 8}},
				[2]point{{x: 5, y: 5}, {x: 8, y: 2}},
			},
			expected: 12,
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
		contents string
		expected vectors
	}{
		"example": {
			contents: `0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2`,
			expected: vectors{
				[2]point{{x: 0, y: 9}, {x: 5, y: 9}},
				[2]point{{x: 8, y: 0}, {x: 0, y: 8}},
				[2]point{{x: 9, y: 4}, {x: 3, y: 4}},
				[2]point{{x: 2, y: 2}, {x: 2, y: 1}},
				[2]point{{x: 7, y: 0}, {x: 7, y: 4}},
				[2]point{{x: 6, y: 4}, {x: 2, y: 0}},
				[2]point{{x: 0, y: 9}, {x: 2, y: 9}},
				[2]point{{x: 3, y: 4}, {x: 1, y: 4}},
				[2]point{{x: 0, y: 0}, {x: 8, y: 8}},
				[2]point{{x: 5, y: 5}, {x: 8, y: 2}},
			},
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			actual := parseContents(test.contents)

			if !reflect.DeepEqual(actual, test.expected) {
				t.Fatalf("Expected: %v\nActual: %v", test.expected, actual)
			}
		})
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
