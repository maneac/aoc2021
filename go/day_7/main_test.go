package main

import "testing"

func TestPart1(t *testing.T) {
	tests := map[string]struct {
		data     []int
		expected int
	}{
		"example": {
			data:     []int{0, 1, 1, 2, 2, 2, 4, 7, 14, 16},
			expected: 37,
		},
		"actual": {
			data:     readData(),
			expected: 340056,
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
			data:     []int{0, 1, 1, 2, 2, 2, 4, 7, 14, 16},
			expected: 168,
		},
		"actual": {
			data:     readData(),
			expected: 96592275,
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
