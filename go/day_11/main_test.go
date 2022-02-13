package main

import (
	"reflect"
	"testing"
)

func TestParseContents(t *testing.T) {
	tests := map[string]struct {
		data     string
		expected Input
	}{
		"example": {
			data: `5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526`,
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

func TestPart1(t *testing.T) {
	tests := map[string]struct {
		data     Input
		expected string
	}{
		"example": {
			data:     exampleData(),
			expected: "1656",
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
			expected: "195",
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

func exampleData() Input {
	data := [100]*octopus{}
	for idx := range data {
		data[idx] = new(octopus)
	}

	// 5483143223
	*data[0] = octopus{5, []*octopus{ /*   */ data[1] /*     */, data[10], data[11]}}
	*data[1] = octopus{4, []*octopus{data[0], data[2], data[10], data[11], data[12]}}
	*data[2] = octopus{8, []*octopus{data[1], data[3], data[11], data[12], data[13]}}
	*data[3] = octopus{3, []*octopus{data[2], data[4], data[12], data[13], data[14]}}
	*data[4] = octopus{1, []*octopus{data[3], data[5], data[13], data[14], data[15]}}
	*data[5] = octopus{4, []*octopus{data[4], data[6], data[14], data[15], data[16]}}
	*data[6] = octopus{3, []*octopus{data[5], data[7], data[15], data[16], data[17]}}
	*data[7] = octopus{2, []*octopus{data[6], data[8], data[16], data[17], data[18]}}
	*data[8] = octopus{2, []*octopus{data[7], data[9], data[17], data[18], data[19]}}
	*data[9] = octopus{3, []*octopus{data[8] /*    */, data[18], data[19] /*     */}}

	// 2745854711
	*data[10] = octopus{2, []*octopus{ /*   */ data[0], data[1] /*     */, data[11] /*     */, data[20], data[21]}}
	*data[11] = octopus{7, []*octopus{data[0], data[1], data[2], data[10], data[12], data[20], data[21], data[22]}}
	*data[12] = octopus{4, []*octopus{data[1], data[2], data[3], data[11], data[13], data[21], data[22], data[23]}}
	*data[13] = octopus{5, []*octopus{data[2], data[3], data[4], data[12], data[14], data[22], data[23], data[24]}}
	*data[14] = octopus{8, []*octopus{data[3], data[4], data[5], data[13], data[15], data[23], data[24], data[25]}}
	*data[15] = octopus{5, []*octopus{data[4], data[5], data[6], data[14], data[16], data[24], data[25], data[26]}}
	*data[16] = octopus{4, []*octopus{data[5], data[6], data[7], data[15], data[17], data[25], data[26], data[27]}}
	*data[17] = octopus{7, []*octopus{data[6], data[7], data[8], data[16], data[18], data[26], data[27], data[28]}}
	*data[18] = octopus{1, []*octopus{data[7], data[8], data[9], data[17], data[19], data[27], data[28], data[29]}}
	*data[19] = octopus{1, []*octopus{data[8], data[9] /*    */, data[18] /*     */, data[28], data[29] /*     */}}

	// 5264556173
	*data[20] = octopus{5, []*octopus{ /*    */ data[10], data[11] /*     */, data[21] /*     */, data[30], data[31]}}
	*data[21] = octopus{2, []*octopus{data[10], data[11], data[12], data[20], data[22], data[30], data[31], data[32]}}
	*data[22] = octopus{6, []*octopus{data[11], data[12], data[13], data[21], data[23], data[31], data[32], data[33]}}
	*data[23] = octopus{4, []*octopus{data[12], data[13], data[14], data[22], data[24], data[32], data[33], data[34]}}
	*data[24] = octopus{5, []*octopus{data[13], data[14], data[15], data[23], data[25], data[33], data[34], data[35]}}
	*data[25] = octopus{5, []*octopus{data[14], data[15], data[16], data[24], data[26], data[34], data[35], data[36]}}
	*data[26] = octopus{6, []*octopus{data[15], data[16], data[17], data[25], data[27], data[35], data[36], data[37]}}
	*data[27] = octopus{1, []*octopus{data[16], data[17], data[18], data[26], data[28], data[36], data[37], data[38]}}
	*data[28] = octopus{7, []*octopus{data[17], data[18], data[19], data[27], data[29], data[37], data[38], data[39]}}
	*data[29] = octopus{3, []*octopus{data[18], data[19] /*     */, data[28] /*     */, data[38], data[39] /*     */}}

	// 6141336146
	*data[30] = octopus{6, []*octopus{ /*    */ data[20], data[21] /*     */, data[31] /*     */, data[40], data[41]}}
	*data[31] = octopus{1, []*octopus{data[20], data[21], data[22], data[30], data[32], data[40], data[41], data[42]}}
	*data[32] = octopus{4, []*octopus{data[21], data[22], data[23], data[31], data[33], data[41], data[42], data[43]}}
	*data[33] = octopus{1, []*octopus{data[22], data[23], data[24], data[32], data[34], data[42], data[43], data[44]}}
	*data[34] = octopus{3, []*octopus{data[23], data[24], data[25], data[33], data[35], data[43], data[44], data[45]}}
	*data[35] = octopus{3, []*octopus{data[24], data[25], data[26], data[34], data[36], data[44], data[45], data[46]}}
	*data[36] = octopus{6, []*octopus{data[25], data[26], data[27], data[35], data[37], data[45], data[46], data[47]}}
	*data[37] = octopus{1, []*octopus{data[26], data[27], data[28], data[36], data[38], data[46], data[47], data[48]}}
	*data[38] = octopus{4, []*octopus{data[27], data[28], data[29], data[37], data[39], data[47], data[48], data[49]}}
	*data[39] = octopus{6, []*octopus{data[28], data[29] /*     */, data[38] /*     */, data[48], data[49] /*     */}}

	// 6357385478
	*data[40] = octopus{6, []*octopus{ /*    */ data[30], data[31] /*     */, data[41] /*     */, data[50], data[51]}}
	*data[41] = octopus{3, []*octopus{data[30], data[31], data[32], data[40], data[42], data[50], data[51], data[52]}}
	*data[42] = octopus{5, []*octopus{data[31], data[32], data[33], data[41], data[43], data[51], data[52], data[53]}}
	*data[43] = octopus{7, []*octopus{data[32], data[33], data[34], data[42], data[44], data[52], data[53], data[54]}}
	*data[44] = octopus{3, []*octopus{data[33], data[34], data[35], data[43], data[45], data[53], data[54], data[55]}}
	*data[45] = octopus{8, []*octopus{data[34], data[35], data[36], data[44], data[46], data[54], data[55], data[56]}}
	*data[46] = octopus{5, []*octopus{data[35], data[36], data[37], data[45], data[47], data[55], data[56], data[57]}}
	*data[47] = octopus{4, []*octopus{data[36], data[37], data[38], data[46], data[48], data[56], data[57], data[58]}}
	*data[48] = octopus{7, []*octopus{data[37], data[38], data[39], data[47], data[49], data[57], data[58], data[59]}}
	*data[49] = octopus{8, []*octopus{data[38], data[39] /*     */, data[48] /*     */, data[58], data[59] /*     */}}

	// 4167524645
	*data[50] = octopus{4, []*octopus{ /*    */ data[40], data[41] /*     */, data[51] /*     */, data[60], data[61]}}
	*data[51] = octopus{1, []*octopus{data[40], data[41], data[42], data[50], data[52], data[60], data[61], data[62]}}
	*data[52] = octopus{6, []*octopus{data[41], data[42], data[43], data[51], data[53], data[61], data[62], data[63]}}
	*data[53] = octopus{7, []*octopus{data[42], data[43], data[44], data[52], data[54], data[62], data[63], data[64]}}
	*data[54] = octopus{5, []*octopus{data[43], data[44], data[45], data[53], data[55], data[63], data[64], data[65]}}
	*data[55] = octopus{2, []*octopus{data[44], data[45], data[46], data[54], data[56], data[64], data[65], data[66]}}
	*data[56] = octopus{4, []*octopus{data[45], data[46], data[47], data[55], data[57], data[65], data[66], data[67]}}
	*data[57] = octopus{6, []*octopus{data[46], data[47], data[48], data[56], data[58], data[66], data[67], data[68]}}
	*data[58] = octopus{4, []*octopus{data[47], data[48], data[49], data[57], data[59], data[67], data[68], data[69]}}
	*data[59] = octopus{5, []*octopus{data[48], data[49] /*     */, data[58] /*     */, data[68], data[69] /*     */}}

	// 2176841721
	*data[60] = octopus{2, []*octopus{ /*    */ data[50], data[51] /*     */, data[61] /*     */, data[70], data[71]}}
	*data[61] = octopus{1, []*octopus{data[50], data[51], data[52], data[60], data[62], data[70], data[71], data[72]}}
	*data[62] = octopus{7, []*octopus{data[51], data[52], data[53], data[61], data[63], data[71], data[72], data[73]}}
	*data[63] = octopus{6, []*octopus{data[52], data[53], data[54], data[62], data[64], data[72], data[73], data[74]}}
	*data[64] = octopus{8, []*octopus{data[53], data[54], data[55], data[63], data[65], data[73], data[74], data[75]}}
	*data[65] = octopus{4, []*octopus{data[54], data[55], data[56], data[64], data[66], data[74], data[75], data[76]}}
	*data[66] = octopus{1, []*octopus{data[55], data[56], data[57], data[65], data[67], data[75], data[76], data[77]}}
	*data[67] = octopus{7, []*octopus{data[56], data[57], data[58], data[66], data[68], data[76], data[77], data[78]}}
	*data[68] = octopus{2, []*octopus{data[57], data[58], data[59], data[67], data[69], data[77], data[78], data[79]}}
	*data[69] = octopus{1, []*octopus{data[58], data[59] /*     */, data[68] /*     */, data[78], data[79] /*     */}}

	// 6882881134
	*data[70] = octopus{6, []*octopus{ /*    */ data[60], data[61] /*     */, data[71] /*     */, data[80], data[81]}}
	*data[71] = octopus{8, []*octopus{data[60], data[61], data[62], data[70], data[72], data[80], data[81], data[82]}}
	*data[72] = octopus{8, []*octopus{data[61], data[62], data[63], data[71], data[73], data[81], data[82], data[83]}}
	*data[73] = octopus{2, []*octopus{data[62], data[63], data[64], data[72], data[74], data[82], data[83], data[84]}}
	*data[74] = octopus{8, []*octopus{data[63], data[64], data[65], data[73], data[75], data[83], data[84], data[85]}}
	*data[75] = octopus{8, []*octopus{data[64], data[65], data[66], data[74], data[76], data[84], data[85], data[86]}}
	*data[76] = octopus{1, []*octopus{data[65], data[66], data[67], data[75], data[77], data[85], data[86], data[87]}}
	*data[77] = octopus{1, []*octopus{data[66], data[67], data[68], data[76], data[78], data[86], data[87], data[88]}}
	*data[78] = octopus{3, []*octopus{data[67], data[68], data[69], data[77], data[79], data[87], data[88], data[89]}}
	*data[79] = octopus{4, []*octopus{data[68], data[69] /*     */, data[78] /*     */, data[88], data[89] /*     */}}

	// 4846848554
	*data[80] = octopus{4, []*octopus{ /*    */ data[70], data[71] /*     */, data[81] /*     */, data[90], data[91]}}
	*data[81] = octopus{8, []*octopus{data[70], data[71], data[72], data[80], data[82], data[90], data[91], data[92]}}
	*data[82] = octopus{4, []*octopus{data[71], data[72], data[73], data[81], data[83], data[91], data[92], data[93]}}
	*data[83] = octopus{6, []*octopus{data[72], data[73], data[74], data[82], data[84], data[92], data[93], data[94]}}
	*data[84] = octopus{8, []*octopus{data[73], data[74], data[75], data[83], data[85], data[93], data[94], data[95]}}
	*data[85] = octopus{4, []*octopus{data[74], data[75], data[76], data[84], data[86], data[94], data[95], data[96]}}
	*data[86] = octopus{8, []*octopus{data[75], data[76], data[77], data[85], data[87], data[95], data[96], data[97]}}
	*data[87] = octopus{5, []*octopus{data[76], data[77], data[78], data[86], data[88], data[96], data[97], data[98]}}
	*data[88] = octopus{5, []*octopus{data[77], data[78], data[79], data[87], data[89], data[97], data[98], data[99]}}
	*data[89] = octopus{4, []*octopus{data[78], data[79] /*     */, data[88] /*     */, data[98], data[99] /*     */}}

	// 5283751526
	*data[90] = octopus{5, []*octopus{ /*    */ data[80], data[81] /*     */, data[91]}}
	*data[91] = octopus{2, []*octopus{data[80], data[81], data[82], data[90], data[92]}}
	*data[92] = octopus{8, []*octopus{data[81], data[82], data[83], data[91], data[93]}}
	*data[93] = octopus{3, []*octopus{data[82], data[83], data[84], data[92], data[94]}}
	*data[94] = octopus{7, []*octopus{data[83], data[84], data[85], data[93], data[95]}}
	*data[95] = octopus{5, []*octopus{data[84], data[85], data[86], data[94], data[96]}}
	*data[96] = octopus{1, []*octopus{data[85], data[86], data[87], data[95], data[97]}}
	*data[97] = octopus{5, []*octopus{data[86], data[87], data[88], data[96], data[98]}}
	*data[98] = octopus{2, []*octopus{data[87], data[88], data[89], data[97], data[99]}}
	*data[99] = octopus{6, []*octopus{data[88], data[89] /*     */, data[98] /*     */}}

	return data
}

func BenchmarkReadData(b *testing.B) {
	for i := 0; i < b.N; i++ {
		if readData("../../data")[0] == nil {
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
