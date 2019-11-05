package day07

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/07"

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
	for _, tc := range []testcase.TestCase{
		testcase.FromString("example", example, "72"),
		testcase.FromFile(t, inputFile, "3176"),
	} {
		testcase.Run(t, tc, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part1)
}

func TestPart2(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.FromFile(t, inputFile, "14710"),
	} {
		testcase.Run(t, tc, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part2)
}
