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
	for _, tc := range []testcase.TestCase2{
		testcase.NewFile(part1Example1File, part1Example1File, "4"),
		testcase.NewFile(part1Example2File, part1Example2File, "7"),
		testcase.NewFile(inputFile, inputFile, "518"),
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
		testcase.NewFile(part2Example1File, part2Example1File, "2"),
		testcase.NewFile(part2Example2File, part2Example2File, "5"),
		testcase.NewFile(inputFile, inputFile, "200"),
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
