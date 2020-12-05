package day08

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/08"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "1342")
	tcPart2 = testcase.NewFile("input", inputFile, "2074")
)

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.New("example1", `""`, "2"),
		testcase.New("example2", `"abc"`, "2"),
		testcase.New("example3", `"aaa\"aaa"`, "3"),
		testcase.New("example4", `"\x27"`, "5"),
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
		testcase.New("example1", `""`, "4"),
		testcase.New("example2", `"abc"`, "4"),
		testcase.New("example3", `"aaa\"aaa"`, "6"),
		testcase.New("example4", `"\x27"`, "5"),
		tcPart2,
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
