package main

import "testing"

func TestPart1(t *testing.T) {
	tests := map[string]struct {
		data     []string
		expected string
	}{
		"example": {
			data: []string{
				"forward 5",
				"down 5",
				"forward 8",
				"up 3",
				"down 8",
				"forward 2",
			},
			expected: "150",
		},
		"actual": {
			data:     readData(),
			expected: "1840243",
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
		data     []string
		expected string
	}{
		"example": {
			data: []string{
				"forward 5",
				"down 5",
				"forward 8",
				"up 3",
				"down 8",
				"forward 2",
			},
			expected: "900",
		},
		"actual": {
			data:     readData(),
			expected: "1727785422",
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
