package day18

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/18"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "21993583522852")
	tcPart2 = testcase.NewFile("input", inputFile, "")
)

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.New("example1", "1 + 2 * 3 + 4 * 5 + 6", "71"),
		testcase.New("example2", "1 + (2 * 3) + (4 * (5 + 6))", "51"),
		testcase.New("example3", "2 * 3 + (4 * 5)", "26"),
		testcase.New("example4", "5 + (8 * 3 + 9 + 3 * 4 * 3)", "437"),
		testcase.New("example5", "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", "12240"),
		testcase.New("example6", "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", "13632"),
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
		testcase.New("example1", "1 + 2 * 3 + 4 * 5 + 6", "231"),
		testcase.New("example2", "1 + (2 * 3) + (4 * (5 + 6))", "51"),
		testcase.New("example3", "2 * 3 + (4 * 5)", "46"),
		testcase.New("example4", "5 + (8 * 3 + 9 + 3 * 4 * 3)", "1445"),
		testcase.New("example5", "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", "669060"),
		testcase.New("example6", "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", "23340"),
		tcPart2,
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
