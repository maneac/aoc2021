package main

import (
	"reflect"
	"testing"
)

const (
	part1Solution = 266301
	part2Solution = 3404870164
)

func TestPart1(t *testing.T) {
	tests := map[string]struct {
		data     []string
		expected int
	}{
		"example": {
			data:     exampleData(),
			expected: 26397,
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
		data     []string
		expected int
	}{
		"example": {
			data:     exampleData(),
			expected: 288957,
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

func TestParseContents(t *testing.T) {
	tests := map[string]struct {
		data     string
		expected []string
	}{
		"example": {
			data: `[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]`,
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

func exampleData() []string {
	return []string{
		"[({(<(())[]>[[{[]{<()<>>",
		"[(()[<>])]({[<{<<[]>>(",
		"{([(<{}[<>[]}>{[]{[(<()>",
		"(((({<>}<{<{<>}{[]{[]{}",
		"[[<[([]))<([[{}[[()]]]",
		"[{[{({}]{}}([{[{{{}}([]",
		"{<[[]]>}<{[{[{[]{()[[[]",
		"[<(<(<(<{}))><([]([]()",
		"<{([([[(<>()){}]>(<<{{",
		"<{([{{}}[<[[[<>{}]]]>[]]",
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
