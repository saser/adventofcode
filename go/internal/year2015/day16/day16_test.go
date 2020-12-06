package day16

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/16"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "")
	tcPart2 = testcase.NewFile("input", inputFile, "")
)

func TestPart1(t *testing.T) {
	tc := testcase.NewFile(inputFile, inputFile, "103")
	tc.Test(t, Part1)
}

func BenchmarkPart1(b *testing.B) {
	tcPart1.Benchmark(b, Part1)
}

func TestPart2(t *testing.T) {
	tc := testcase.NewFile(inputFile, inputFile, "405")
	tc.Test(t, Part2)
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
