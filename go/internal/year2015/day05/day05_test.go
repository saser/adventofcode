package day05

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/05"

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.FromString("example1", "ugknbfddgicrmopn", "1"),
		testcase.FromString("example2", "aaa", "1"),
		testcase.FromString("example3", "jchzalrnumimnmhp", "0"),
		testcase.FromString("example4", "haegwjzuvuyypxyu", "0"),
		testcase.FromString("example5", "dvszwmarrgswjxmb", "0"),
		testcase.FromFile(t, inputFile, "255"),
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
		testcase.FromString("example1", "qjhvhtzxzqqjkmpb", "1"),
		testcase.FromString("example2", "xxyxx", "1"),
		testcase.FromString("example3", "uurcxstgmygtbstg", "0"),
		testcase.FromString("example4", "ieodomkazucvgmuy", "0"),
		testcase.FromFile(t, inputFile, "55"),
	} {
		testcase.Run(t, tc, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part2)
}
