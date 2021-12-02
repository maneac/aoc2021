package main

import "testing"

func TestPart1(t *testing.T) {
	tests := map[string]struct {
		data     []int
		expected string
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
			expected: "7",
		},
		"actual": {
			data:     readData(),
			expected: "1374",
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
		expected string
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
			expected: "5",
		},
		"actual": {
			data:     readData(),
			expected: "1418",
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
