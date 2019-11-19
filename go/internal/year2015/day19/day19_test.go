package day19

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const (
	exampleFile1 = "testdata/example1"
	exampleFile2 = "testdata/example2"
	inputFile    = "../testdata/19"
)

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.FromFile(t, exampleFile1, "4"),
		testcase.FromFile(t, exampleFile2, "7"),
		testcase.FromFile(t, inputFile, ""),
	} {
		testcase.Run(t, tc, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part1)
}
