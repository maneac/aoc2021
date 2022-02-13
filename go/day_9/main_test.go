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
		"eample": {
			data: `2199943210
3987894921
9856789892
8767896789
9899965678`,
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
			expected: "15",
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
			expected: "1134",
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
		heightmap: map[point]int{
			{x: 0, y: 0}: 2,
			{x: 1, y: 0}: 1,
			{x: 2, y: 0}: 9,
			{x: 3, y: 0}: 9,
			{x: 4, y: 0}: 9,
			{x: 5, y: 0}: 4,
			{x: 6, y: 0}: 3,
			{x: 7, y: 0}: 2,
			{x: 8, y: 0}: 1,
			{x: 9, y: 0}: 0,
			{x: 0, y: 1}: 3,
			{x: 1, y: 1}: 9,
			{x: 2, y: 1}: 8,
			{x: 3, y: 1}: 7,
			{x: 4, y: 1}: 8,
			{x: 5, y: 1}: 9,
			{x: 6, y: 1}: 4,
			{x: 7, y: 1}: 9,
			{x: 8, y: 1}: 2,
			{x: 9, y: 1}: 1,
			{x: 0, y: 2}: 9,
			{x: 1, y: 2}: 8,
			{x: 2, y: 2}: 5,
			{x: 3, y: 2}: 6,
			{x: 4, y: 2}: 7,
			{x: 5, y: 2}: 8,
			{x: 6, y: 2}: 9,
			{x: 7, y: 2}: 8,
			{x: 8, y: 2}: 9,
			{x: 9, y: 2}: 2,
			{x: 0, y: 3}: 8,
			{x: 1, y: 3}: 7,
			{x: 2, y: 3}: 6,
			{x: 3, y: 3}: 7,
			{x: 4, y: 3}: 8,
			{x: 5, y: 3}: 9,
			{x: 6, y: 3}: 6,
			{x: 7, y: 3}: 7,
			{x: 8, y: 3}: 8,
			{x: 9, y: 3}: 9,
			{x: 0, y: 4}: 9,
			{x: 1, y: 4}: 8,
			{x: 2, y: 4}: 9,
			{x: 3, y: 4}: 9,
			{x: 4, y: 4}: 9,
			{x: 5, y: 4}: 6,
			{x: 6, y: 4}: 5,
			{x: 7, y: 4}: 6,
			{x: 8, y: 4}: 7,
			{x: 9, y: 4}: 8,
		},
		maxX: 9,
		maxY: 4,
	}
}

func BenchmarkReadData(b *testing.B) {
	for i := 0; i < b.N; i++ {
		if readData("../../data").heightmap == nil {
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
