package day10

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const (
	inputFile    = "../testdata/10"
	example1File = "testdata/example1"
	example2File = "testdata/example2"
)

var (
	tcPart1 = testcase.NewFile("input", inputFile, "")
	tcPart2 = testcase.NewFile("input", inputFile, "")
)

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.NewFile("example1", example1File, "35"),
		testcase.NewFile("example2", example2File, "220"),
		tcPart1,
	} {
		tc.Test(t, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tcPart1.Benchmark(b, Part1)
}

func TestPart2(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		tcPart2,
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
