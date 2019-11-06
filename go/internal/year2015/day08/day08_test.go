package day08

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/08"

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.FromString("example1", `""`, "2"),
		testcase.FromString("example2", `"abc"`, "2"),
		testcase.FromString("example3", `"aaa\"aaa"`, "3"),
		testcase.FromString("example4", `"\x27"`, "5"),
		testcase.FromFile(t, inputFile, "1342"),
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
		testcase.FromString("example1", `""`, "4"),
		testcase.FromString("example2", `"abc"`, "4"),
		testcase.FromString("example3", `"aaa\"aaa"`, "6"),
		testcase.FromString("example4", `"\x27"`, "5"),
		testcase.FromFile(t, inputFile, "2074"),
	} {
		testcase.Run(t, tc, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part2)
}
