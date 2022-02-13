package main

import "testing"

func TestPart1(t *testing.T) {
	tests := map[string]struct {
		data     Input
		expected string
	}{
		"example": {
			data:     Input{0, 1, 1, 2, 2, 2, 4, 7, 14, 16},
			expected: "37",
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
			data:     Input{0, 1, 1, 2, 2, 2, 4, 7, 14, 16},
			expected: "168",
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
