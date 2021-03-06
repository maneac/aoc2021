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
		"example": {
			data: `6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5`,
			expected: exampleData(),
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			actual := parseContents(test.data)

			if !reflect.DeepEqual(test.expected, actual) {
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
			expected: "17",
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
			expected: "0",
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
		points: map[point]struct{}{
			{x: 6, y: 10}:  {},
			{x: 0, y: 14}:  {},
			{x: 9, y: 10}:  {},
			{x: 0, y: 3}:   {},
			{x: 10, y: 4}:  {},
			{x: 4, y: 11}:  {},
			{x: 6, y: 0}:   {},
			{x: 6, y: 12}:  {},
			{x: 4, y: 1}:   {},
			{x: 0, y: 13}:  {},
			{x: 10, y: 12}: {},
			{x: 3, y: 4}:   {},
			{x: 3, y: 0}:   {},
			{x: 8, y: 4}:   {},
			{x: 1, y: 10}:  {},
			{x: 2, y: 14}:  {},
			{x: 8, y: 10}:  {},
			{x: 9, y: 0}:   {},
		},
		folds: []point{
			{x: 0, y: 7},
			{x: 5, y: 0},
		},
	}
}

func BenchmarkReadData(b *testing.B) {
	for i := 0; i < b.N; i++ {
		if readData("../../data").points == nil {
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
