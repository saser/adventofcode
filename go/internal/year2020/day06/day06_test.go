package day06

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/06"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "6521")
	tcPart2 = testcase.NewFile("input", inputFile, "3305")
)

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase2{
		testcase.NewFile("testdata/example", "testdata/example", "11"),
		tcPart1,
	} {
		tc.Test(t, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tcPart1.Benchmark(b, Part1)
}

func TestPart2(t *testing.T) {
	for _, tc := range []testcase.TestCase2{
		testcase.NewFile("testdata/example", "testdata/example", "6"),
		tcPart2,
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
