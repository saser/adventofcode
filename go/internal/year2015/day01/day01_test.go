package day01

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/01"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "232")
	tcPart2 = testcase.NewFile("input", inputFile, "1783")
)

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.New("example1_1", "(())", "0"),
		testcase.New("example1_2", "()()", "0"),
		testcase.New("example2_1", "(((", "3"),
		testcase.New("example2_2", "(()(()(", "3"),
		testcase.New("example3", "))(((((", "3"),
		testcase.New("example4_1", "())", "-1"),
		testcase.New("example4_2", "))(", "-1"),
		testcase.New("example5_1", ")))", "-3"),
		testcase.New("example5_2", ")())())", "-3"),
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
		testcase.New("example1", ")", "1"),
		testcase.New("example2", "()())", "5"),
		tcPart2,
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
