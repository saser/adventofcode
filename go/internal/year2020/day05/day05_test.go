package day05

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/05"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "832")
	tcPart2 = testcase.NewFile("input", inputFile, "517")
)

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
	for _, tc := range []testcase.TestCase2{
		testcase.New("example1", "FBFBBFFRLR", "357"),
		testcase.New("example2", "BFFFBBFRRR", "567"),
		testcase.New("example3", "FFFBBBFRRR", "119"),
		testcase.New("example4", "BBFFBBFRLL", "820"),
		tcPart1,
	} {
		tc.Test(t, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tcPart1.Benchmark(b, Part1)
}

func TestPart2(t *testing.T) {
	for _, tc := range []testcase.TestCase2{
		tcPart2,
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
