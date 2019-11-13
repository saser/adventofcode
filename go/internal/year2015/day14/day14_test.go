package day14

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/14"

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.FromFile(t, inputFile, "2696"),
	} {
		testcase.Run(t, tc, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part1)
}
