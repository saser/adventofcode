package day04

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/04"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "237")
	tcPart2 = testcase.NewFile("input", inputFile, "172")
)

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.NewFile("testdata/p1example", "testdata/p1example", "2"),
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
		testcase.NewFile("testdata/p2example_invalid", "testdata/p2example_invalid", "0"),
		testcase.NewFile("testdata/p2example_valid", "testdata/p2example_valid", "4"),
		tcPart2,
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
