package main

import (
	"testing"
)

func TestPart1(t *testing.T) {
	tests := map[string]struct {
		data     *treeNode
		expected string
	}{
		"example": {
			data:     exampleBalancedTree(),
			expected: "198",
		},
		"actual": {
			data:     readData(),
			expected: "3148794",
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
		data     *treeNode
		expected string
	}{
		"example": {
			data:     exampleBalancedTree(),
			expected: "230",
		},
		"actual": {
			data:     readData(),
			expected: "2795310",
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
		contents string
		expected *treeNode
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
			expected: exampleTree(),
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			actual := parseContents(test.contents)
			if actual.String() != test.expected.String() {
				t.Fatalf("Expected: %v\nActual: %v", test.expected, actual)
			}
		})
	}
}

func TestBalance(t *testing.T) {
	tests := map[string]struct {
		input    *treeNode
		expected *treeNode
	}{
		"example": {
			input:    exampleTree(),
			expected: exampleBalancedTree(),
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

func exampleTree() *treeNode {
	return &treeNode{childCount: 12, children: [2]*treeNode{
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
	}}
}

func exampleBalancedTree() *treeNode {
	return &treeNode{childCount: 12, children: [2]*treeNode{
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
	}}
}
