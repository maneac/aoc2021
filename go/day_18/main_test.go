package main

import (
	"reflect"
	"testing"
)

func TestParseContents(t *testing.T) {
	tests := map[string]struct {
		Input    string
		expected Input
	}{
		"explodeExample1": {
			Input: "[[[[[9,8],1],2],3],4]",
			expected: func() Input {
				l := newSnailfishList()

				l.PushBack(&snailfish{
					level: 4,
					value: 9,
				})
				l.PushBack(&snailfish{
					level: 4,
					value: 8,
				})
				l.PushBack(&snailfish{
					level: 3,
					value: 1,
				})
				l.PushBack(&snailfish{
					level: 2,
					value: 2,
				})
				l.PushBack(&snailfish{
					level: 1,
					value: 3,
				})
				l.PushBack(&snailfish{
					level: 0,
					value: 4,
				})

				return Input{l}
			}(),
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			actual := parseContents(test.Input)

			if !reflect.DeepEqual(actual, test.expected) {
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
		"singlePairExample1": {
			data:     parseContents("[9,1]"),
			expected: "29",
		},
		"singlePairExample2": {
			data:     parseContents("[1,9]"),
			expected: "21",
		},
		"nestedSinglePairsExample": {
			data:     parseContents("[[9,1],[1,9]]"),
			expected: "129",
		},
		"doubleNested": {
			data:     parseContents("[[9,[1,0]],[1,9]]"),
			expected: "141",
		},
		"example1": {
			data:     parseContents("[[1,2],[[3,4],5]]"),
			expected: "143",
		},
		"example2": {
			data:     parseContents("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"),
			expected: "1384",
		},
		"example3": {
			data:     parseContents("[[[[1,1],[2,2]],[3,3]],[4,4]]"),
			expected: "445",
		},
		"example4": {
			data:     parseContents("[[[[3,0],[5,3]],[4,4]],[5,5]]"),
			expected: "791",
		},
		"example5": {
			data:     parseContents("[[[[5,0],[7,4]],[5,5]],[6,6]]"),
			expected: "1137",
		},
		"example6": {
			data:     parseContents("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"),
			expected: "3488",
		},
		"largeExampleReduced": {
			data:     parseContents("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"),
			expected: "4140",
		},
		"largeExampleExpanded": {
			data: parseContents(`[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]`),
			expected: "4140",
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
			data: parseContents(`[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]`),
			expected: "3993",
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

func TestReduceOnce(t *testing.T) {
	tests := map[string]struct {
		data     *snailfishList
		expected *snailfishList
	}{
		"explodeExample1": {
			data:     listFromLine("[[[[[9,8],1],2],3],4]"),
			expected: listFromLine("[[[[0,9],2],3],4]"),
		},
		"explodeExample2": {
			data:     listFromLine("[7,[6,[5,[4,[3,2]]]]]"),
			expected: listFromLine("[7,[6,[5,[7,0]]]]"),
		},
		"explodeExample3": {
			data:     listFromLine("[[6,[5,[4,[3,2]]]],1]"),
			expected: listFromLine("[[6,[5,[7,0]]],3]"),
		},
		"explodeExample4": {
			data:     listFromLine("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"),
			expected: listFromLine("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
		},
		"explodeExample5": {
			data:     listFromLine("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
			expected: listFromLine("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"),
		},
		"splitExample1": {
			data:     listFromLine("[[[[0,7],4],[15,[0,13]]],[1,1]]"),
			expected: listFromLine("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"),
		},
		"splitExample2": {
			data:     listFromLine("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"),
			expected: listFromLine("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"),
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			actual := *test.data
			_ = (&actual).reduceOnce()

			if !reflect.DeepEqual(&actual, test.expected) {
				t.Fatalf("\nExpected: %v\nActual:   %v", test.expected, &actual)
			}
		})
	}
}

func TestReduceInput(t *testing.T) {
	tests := map[string]struct {
		data     Input
		expected *snailfishList
	}{
		"additionExample1": {
			data: parseContents(`[1,2]
[[3,4],5]`),
			expected: listFromLine("[[1,2],[[3,4],5]]"),
		},
		"splitExample": {
			data: parseContents(`[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]`),
			expected: listFromLine("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"),
		},
		"sumExample1": {
			data: parseContents(`[1,1]
[2,2]
[3,3]
[4,4]`),
			expected: listFromLine("[[[[1,1],[2,2]],[3,3]],[4,4]]"),
		},
		"sumExample2": {
			data: parseContents(`[1,1]
[2,2]
[3,3]
[4,4]
[5,5]`),
			expected: listFromLine("[[[[3,0],[5,3]],[4,4]],[5,5]]"),
		},
		"sumExample3": {
			data: parseContents(`[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]`),
			expected: listFromLine("[[[[5,0],[7,4]],[5,5]],[6,6]]"),
		},
		"sumExample4": {
			data: parseContents(`[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]`),
			expected: listFromLine("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"),
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			actual := test.data.reduce()

			if !reflect.DeepEqual(actual, test.expected) {
				t.Fatalf("\nExpected: %v\nActual:   %v", test.expected, actual)
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
	for i := 0; i < b.N; i++ {
		b.StopTimer()
		data := readData("../../data")
		b.StartTimer()
		if data.Part1() != part1Solution {
			b.FailNow()
		}
	}
}

func BenchmarkPart2(b *testing.B) {
	for i := 0; i < b.N; i++ {
		b.StopTimer()
		data := readData("../../data")
		b.StartTimer()
		if data.Part2() != part2Solution {
			b.FailNow()
		}
	}
}
