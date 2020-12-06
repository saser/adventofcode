package day07

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/07"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "3176")
	tcPart2 = testcase.NewFile("input", inputFile, "14710")
)

func TestPart1(t *testing.T) {
	// Slightly modified from the example given in the description: wire `d` was renamed to `a` in order to have some
	// value to test against.
	example := `123 -> x
456 -> y
x AND y -> a
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i`
	for _, tc := range []testcase.TestCase2{
		testcase.New("example", example, "72"),
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
		tcPart2,
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
