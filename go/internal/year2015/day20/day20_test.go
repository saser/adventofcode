package day20

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/20"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "665280")
	tcPart2 = testcase.NewFile("input", inputFile, "705600")
)

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase2{
		testcase.New("example1", "10", "1"),
		testcase.New("example2", "70", "4"),
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
		testcase.New("example1", "10", "1"),
		testcase.New("example2", "70", "4"),
		tcPart2,
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
