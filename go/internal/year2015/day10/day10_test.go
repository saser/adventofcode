package day10

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/10"

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.FromString("example1", "1", "11"),
		testcase.FromString("example2", "11", "21"),
		testcase.FromString("example3", "21", "1211"),
		testcase.FromString("example4", "1211", "111221"),
		testcase.FromString("example5", "111221", "312211"),
	} {
		testcase.Run(t, tc, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part1)
}
