package main

import (
	"reflect"
	"testing"
)

func TestReadData(t *testing.T) {
	data := readData("../../data")
	if data == nil {
		t.FailNow()
	}
}

func TestParseContents(t *testing.T) {
	tests := map[string]struct {
		input    string
		expected Input
	}{
		"example": {
			input: `forward 5
down 5
forward 8
up 3
down 8
forward 2`,
			expected: exampleData(),
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			actual := parseContents(test.input)

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
			expected: "150",
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
			expected: "900",
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
	return Input([]string{
		"forward 5",
		"down 5",
		"forward 8",
		"up 3",
		"down 8",
		"forward 2",
	})
}

func BenchmarkReadData(b *testing.B) {
	for i := 0; i < b.N; i++ {
		if readData("../../data") == nil {
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
