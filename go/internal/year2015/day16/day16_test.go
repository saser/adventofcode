package day16

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/16"

func TestPart1(t *testing.T) {
	tc := testcase.FromFile(t, inputFile, "103")
	testcase.Run(t, tc, Part1)
}

func BenchmarkPart1(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part1)
}
