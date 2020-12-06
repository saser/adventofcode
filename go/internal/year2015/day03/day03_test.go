package day03

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/03"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "")
	tcPart2 = testcase.NewFile("input", inputFile, "")
)

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase2{
		testcase.New("example1", ">", "2"),
		testcase.New("example2", "^>v<", "4"),
		testcase.New("example3", "^v^v^v^v^v", "2"),
		testcase.NewFile(inputFile, inputFile, "2572"),
	} {
		tc.Test(t, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tcPart1.Benchmark(b, Part1)
}

func TestPart2(t *testing.T) {
	for _, tc := range []testcase.TestCase2{
		testcase.New("example1", "^v", "3"),
		testcase.New("example2", "^>v<", "3"),
		testcase.New("example3", "^v^v^v^v^v", "11"),
		testcase.NewFile(inputFile, inputFile, "2631"),
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
