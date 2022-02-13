package main

import (
	"reflect"
	"testing"
)

func TestReadData(t *testing.T) {
	data := readData("../../data")
	if data.childCount == 0 {
		t.FailNow()
	}
}

func TestParseContents(t *testing.T) {
	tests := map[string]struct {
		contents string
		expected Input
	}{
		"example": {
			contents: `00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010`,
			expected: exampleDataUnbalanced(),
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			actual := parseContents(test.contents)
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
			expected: "198",
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
			expected: "230",
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

func TestBalance(t *testing.T) {
	tests := map[string]struct {
		input    treeNode
		expected treeNode
	}{
		"example": {
			input:    (treeNode)(exampleDataUnbalanced()),
			expected: (treeNode)(exampleData()),
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			test.input.balance()
			if test.input.String() != test.expected.String() {
				t.Fatalf(`
	Expected: %v
	Actual:   %v
`, test.expected, test.input)
			}
		})
	}
}

func exampleDataUnbalanced() Input {
	return (Input)(treeNode{childCount: 12, children: [2]*treeNode{
		{value: 0, childCount: 5, children: [2]*treeNode{
			{value: 0, childCount: 3, children: [2]*treeNode{
				{value: 0, childCount: 1, children: [2]*treeNode{
					nil,
					{value: 1, childCount: 1, children: [2]*treeNode{
						{value: 0b00010},
						nil,
					}},
				}},
				{value: 1, childCount: 2, children: [2]*treeNode{
					{value: 0, childCount: 1, children: [2]*treeNode{
						{value: 0b00100},
						nil,
					}},
					{value: 1, childCount: 1, children: [2]*treeNode{
						nil,
						{value: 0b00111},
					}},
				}},
			}},
			{value: 1, childCount: 2, children: [2]*treeNode{
				{value: 0, childCount: 1, children: [2]*treeNode{
					nil,
					{value: 1, childCount: 1, children: [2]*treeNode{
						{value: 0b01010},
						nil,
					}},
				}},
				{value: 1, childCount: 1, children: [2]*treeNode{
					nil,
					{value: 1, childCount: 1, children: [2]*treeNode{
						nil,
						{value: 0b01111},
					}},
				}},
			}},
		}},
		{value: 1, childCount: 7, children: [2]*treeNode{
			{value: 0, childCount: 4, children: [2]*treeNode{
				{value: 0, childCount: 1, children: [2]*treeNode{
					{value: 0, childCount: 1, children: [2]*treeNode{
						{value: 0b10000},
						nil,
					}},
					nil,
				}},
				{value: 1, childCount: 3, children: [2]*treeNode{
					{value: 0, childCount: 1, children: [2]*treeNode{
						nil,
						{value: 0b10101},
					}},
					{value: 1, childCount: 2, children: [2]*treeNode{
						{value: 0b10110},
						{value: 0b10111},
					}},
				}},
			}},
			{value: 1, childCount: 3, children: [2]*treeNode{
				{value: 0, childCount: 1, children: [2]*treeNode{
					{value: 0, childCount: 1, children: [2]*treeNode{
						nil,
						{value: 0b11001},
					}},
					nil,
				}},
				{value: 1, childCount: 2, children: [2]*treeNode{
					{value: 0, childCount: 1, children: [2]*treeNode{
						{value: 0b11100},
						nil,
					}},
					{value: 1, childCount: 1, children: [2]*treeNode{
						{value: 0b11110},
						nil,
					}},
				}},
			}},
		}},
	}})
}

func exampleData() Input {
	return (Input)(treeNode{childCount: 12, children: [2]*treeNode{
		{value: 0, childCount: 5, children: [2]*treeNode{
			{value: 1, childCount: 2, children: [2]*treeNode{
				{value: 0, childCount: 1, children: [2]*treeNode{
					nil,
					{value: 1, childCount: 1, children: [2]*treeNode{
						{value: 0b01010},
						nil,
					}},
				}},
				{value: 1, childCount: 1, children: [2]*treeNode{
					nil,
					{value: 1, childCount: 1, children: [2]*treeNode{
						nil,
						{value: 0b01111},
					}},
				}},
			}},
			{value: 0, childCount: 3, children: [2]*treeNode{
				{value: 0, childCount: 1, children: [2]*treeNode{
					nil,
					{value: 1, childCount: 1, children: [2]*treeNode{
						{value: 0b00010},
						nil,
					}},
				}},
				{value: 1, childCount: 2, children: [2]*treeNode{
					{value: 0, childCount: 1, children: [2]*treeNode{
						{value: 0b00100},
						nil,
					}},
					{value: 1, childCount: 1, children: [2]*treeNode{
						nil,
						{value: 0b00111},
					}},
				}},
			}},
		}},
		{value: 1, childCount: 7, children: [2]*treeNode{
			{value: 1, childCount: 3, children: [2]*treeNode{
				{value: 0, childCount: 1, children: [2]*treeNode{
					nil,
					{value: 0, childCount: 1, children: [2]*treeNode{
						nil,
						{value: 0b11001},
					}},
				}},
				{value: 1, childCount: 2, children: [2]*treeNode{
					{value: 0, childCount: 1, children: [2]*treeNode{
						{value: 0b11100},
						nil,
					}},
					{value: 1, childCount: 1, children: [2]*treeNode{
						{value: 0b11110},
						nil,
					}},
				}},
			}},
			{value: 0, childCount: 4, children: [2]*treeNode{
				{value: 0, childCount: 1, children: [2]*treeNode{
					nil,
					{value: 0, childCount: 1, children: [2]*treeNode{
						{value: 0b10000},
						nil,
					}},
				}},
				{value: 1, childCount: 3, children: [2]*treeNode{
					{value: 0, childCount: 1, children: [2]*treeNode{
						nil,
						{value: 0b10101},
					}},
					{value: 1, childCount: 2, children: [2]*treeNode{
						{value: 0b10110},
						{value: 0b10111},
					}},
				}},
			}},
		}},
	}})
}

func BenchmarkReadData(b *testing.B) {
	for i := 0; i < b.N; i++ {
		if readData("../../data").childCount == 0 {
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
