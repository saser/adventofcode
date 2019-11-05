package day06

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/06"

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.FromString("example1", "turn on 0,0 through 999,999", "1000000"),
		testcase.FromString("example2", "toggle 0,0 through 999,0", "1000"),
		testcase.FromString("example3", "turn off 499,499 through 500,500", "0"),
		testcase.FromFile(t, inputFile, "569999"),
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
		testcase.FromString("example1", "turn on 0,0 through 0,0", "1"),
		testcase.FromString("example2", "toggle 0,0 through 999,999", "2000000"),
		testcase.FromFile(t, inputFile, "17836115"),
	} {
		testcase.Run(t, tc, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part2)
}
