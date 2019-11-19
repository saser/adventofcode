package day19

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const (
	part1Example1File = "testdata/part1example1"
	part1Example2File = "testdata/part1example2"
	part2Example1File = "testdata/part2example1"
	part2Example2File = "testdata/part2example2"
	inputFile         = "../testdata/19"
)

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.FromFile(t, part1Example1File, "4"),
		testcase.FromFile(t, part1Example2File, "7"),
		testcase.FromFile(t, inputFile, "518"),
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
		testcase.FromFile(t, part2Example1File, "3"),
		testcase.FromFile(t, part2Example2File, "6"),
		testcase.FromFile(t, inputFile, ""),
	} {
		testcase.Run(t, tc, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part2)
}
