package day19

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/19"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "518")
	tcPart2 = testcase.NewFile("input", inputFile, "200")
)

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase2{
		testcase.NewFile("example1", "testdata/part1example1", "4"),
		testcase.NewFile("example2", "testdata/part1example2", "7"),
		tcPart1,
	} {
		tc.Test(t, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tcPart1.Benchmark(b, Part1)
}

func TestPart2(t *testing.T) {
	// The example test cases are off by one. This is due to a discrepancy between the example input and the actual
	// input: in the actual input, all productions from "e" are to two other tokens, while in the example input the
	// productions from "e" are to one other token.
	for _, tc := range []testcase.TestCase2{
		testcase.NewFile("example1", "testdata/part2example1", "2"),
		testcase.NewFile("example2", "testdata/part2example2", "5"),
		tcPart2,
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
