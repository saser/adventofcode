package day05

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/05"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "255")
	tcPart2 = testcase.NewFile("input", inputFile, "55")
)

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.New("example1", "ugknbfddgicrmopn", "1"),
		testcase.New("example2", "aaa", "1"),
		testcase.New("example3", "jchzalrnumimnmhp", "0"),
		testcase.New("example4", "haegwjzuvuyypxyu", "0"),
		testcase.New("example5", "dvszwmarrgswjxmb", "0"),
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
		testcase.New("example1", "qjhvhtzxzqqjkmpb", "1"),
		testcase.New("example2", "xxyxx", "1"),
		testcase.New("example3", "uurcxstgmygtbstg", "0"),
		testcase.New("example4", "ieodomkazucvgmuy", "0"),
		tcPart2,
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
