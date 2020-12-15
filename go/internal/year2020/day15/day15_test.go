package day15

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/15"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "870")
	tcPart2 = testcase.NewFile("input", inputFile, "")
)

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.New("example1", "0,3,6", "436"),
		testcase.New("example2", "1,3,2", "1"),
		testcase.New("example3", "2,1,3", "10"),
		testcase.New("example4", "1,2,3", "27"),
		testcase.New("example5", "2,3,1", "78"),
		testcase.New("example6", "3,2,1", "438"),
		testcase.New("example7", "3,1,2", "1836"),
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
