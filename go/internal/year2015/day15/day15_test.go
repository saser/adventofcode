package day15

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const (
	exampleFile = "testdata/example"
	inputFile   = "../testdata/15"
)

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase2{
		testcase.NewFile(exampleFile, exampleFile, "62842880"),
		testcase.NewFile(inputFile, inputFile, "13882464"),
	} {
		tc.Test(t, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tcPart1.Benchmark(b, Part1)
}

func TestPart2(t *testing.T) {
	for _, tc := range []testcase.TestCase2{
		testcase.NewFile(exampleFile, exampleFile, "57600000"),
		testcase.NewFile(inputFile, inputFile, "11171160"),
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
