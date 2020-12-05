package day05

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/05"

func Test_parse(t *testing.T) {
	for _, tt := range []struct {
		s    string
		want int
	}{
		{s: "FBFBBFFRLR", want: (44 << 3) + 5},
		{s: "BFFFBBFRRR", want: (70 << 3) + 7},
		{s: "FFFBBBFRRR", want: (14 << 3) + 7},
		{s: "BBFFBBFRLL", want: (102 << 3) + 4},
	} {
		if got := parse(tt.s); got != tt.want {
			t.Errorf("parse(%q) = %v; want %v", tt.s, got, tt.want)
		}
	}
}

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.FromString("example1", "FBFBBFFRLR", "357"),
		testcase.FromString("example2", "BFFFBBFRRR", "567"),
		testcase.FromString("example3", "FFFBBBFRRR", "119"),
		testcase.FromString("example4", "BBFFBBFRLL", "820"),
		testcase.FromFile(t, inputFile, "832"),
	} {
		testcase.Run(t, tc, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part1)
}

func TestPart2(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.FromFile(t, inputFile, "517"),
	} {
		testcase.Run(t, tc, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part2)
}
