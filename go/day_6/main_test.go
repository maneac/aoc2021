package main

import (
	"reflect"
	"testing"
)

func TestReadData(t *testing.T) {
	data := readData("../../data")
	if reflect.DeepEqual(data, Input{}) {
		t.FailNow()
	}
}

func TestParseContents(t *testing.T) {
	tests := map[string]struct {
		data     string
		expected Input
	}{
		"example": {
			data:     "3,4,3,1,2",
			expected: Input{0, 1, 1, 2, 1, 0, 0, 0, 0},
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
			data:     Input{0, 1, 1, 2, 1, 0, 0, 0, 0},
			expected: "5934",
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
			data:     Input{0, 1, 1, 2, 1, 0, 0, 0, 0},
			expected: "26984457539",
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

func TestFishAfterDays(t *testing.T) {
	tests := map[string]struct {
		data     Input
		days     int
		expected int
	}{
		"18 days example": {
			data:     Input{0, 1, 1, 2, 1, 0, 0, 0, 0},
			days:     18,
			expected: 26,
		},
		"80 days example": {
			data:     Input{0, 1, 1, 2, 1, 0, 0, 0, 0},
			days:     80,
			expected: 5934,
		},
		"256 days example": {
			data:     Input{0, 1, 1, 2, 1, 0, 0, 0, 0},
			days:     256,
			expected: 26984457539,
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			actual := test.data.fishAfterDays(test.days)

			if actual != test.expected {
				t.Fatalf("Expected: %v\nActual: %v", test.expected, actual)
			}
		})
	}
}

func BenchmarkReadData(b *testing.B) {
	for i := 0; i < b.N; i++ {
		if reflect.DeepEqual(readData("../../data"), Input{}) {
			b.FailNow()
		}
	}
}

func BenchmarkPart1(b *testing.B) {
	data := readData("../../data")
	for i := 0; i < b.N; i++ {
		if data.Part1() != part1Solution {
			b.FailNow()
		}
	}
}

func BenchmarkPart2(b *testing.B) {
	data := readData("../../data")
	for i := 0; i < b.N; i++ {
		if data.Part2() != part2Solution {
			b.FailNow()
		}
	}
}
