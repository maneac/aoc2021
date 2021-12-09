package main

import "testing"

const (
	part1Solution = 1374
	part2Solution = 1418
)

func TestPart1(t *testing.T) {
	tests := map[string]struct {
		data     []int
		expected int
	}{
		"example": {
			data: []int{
				199,
				200,
				208,
				210,
				200,
				207,
				240,
				269,
				260,
				263,
			},
			expected: 7,
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
		data     []int
		expected int
	}{
		"example": {
			data: []int{
				199,
				200,
				208,
				210,
				200,
				207,
				240,
				269,
				260,
				263,
			},
			expected: 5,
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
