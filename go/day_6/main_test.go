package main

import (
	"reflect"
	"testing"
)

func TestPart1(t *testing.T) {
	tests := map[string]struct {
		data     [9]int
		expected int
	}{
		"example": {
			data:     [9]int{0, 1, 1, 2, 1, 0, 0, 0, 0},
			expected: 5934,
		},
		"actual": {
			data:     readData(),
			expected: 360761,
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
		data     [9]int
		expected int
	}{
		"example": {
			data:     [9]int{0, 1, 1, 2, 1, 0, 0, 0, 0},
			expected: 26984457539,
		},
		"actual": {
			data:     readData(),
			expected: 1632779838045,
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
		data     string
		expected [9]int
	}{
		"example": {
			data:     "3,4,3,1,2",
			expected: [9]int{0, 1, 1, 2, 1, 0, 0, 0, 0},
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

func TestFishAfterDays(t *testing.T) {
	tests := map[string]struct {
		data     [9]int
		days     int
		expected int
	}{
		"18 days example": {
			data:     [9]int{0, 1, 1, 2, 1, 0, 0, 0, 0},
			days:     18,
			expected: 26,
		},
		"80 days example": {
			data:     [9]int{0, 1, 1, 2, 1, 0, 0, 0, 0},
			days:     80,
			expected: 5934,
		},
		"256 days example": {
			data:     [9]int{0, 1, 1, 2, 1, 0, 0, 0, 0},
			days:     256,
			expected: 26984457539,
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			actual := fishAfterDays(test.data, test.days)

			if actual != test.expected {
				t.Fatalf("Expected: %v\nActual: %v", test.expected, actual)
			}
		})
	}
}
