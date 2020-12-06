package day02

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/02"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "1586300")
	tcPart2 = testcase.NewFile("input", inputFile, "3737498")
)

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.New("example1", "2x3x4", "58"),
		testcase.New("example2", "1x1x10", "43"),
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
		testcase.New("example1", "2x3x4", "34"),
		testcase.New("example2", "1x1x10", "14"),
		tcPart2,
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
