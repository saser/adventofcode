package day15

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const (
	exampleFile = "testdata/example"
	inputFile   = "../testdata/15"
)

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.FromFile(t, exampleFile, "62842880"),
		testcase.FromFile(t, inputFile, "13882464"),
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
		testcase.FromFile(t, exampleFile, "57600000"),
	} {
		testcase.Run(t, tc, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part2)
}
