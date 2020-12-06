package day01

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/01"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "")
	tcPart2 = testcase.NewFile("input", inputFile, "")
)

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase2{
		testcase.New("example1_1", "(())", "0"),
		testcase.New("example1_2", "()()", "0"),
		testcase.New("example2_1", "(((", "3"),
		testcase.New("example2_2", "(()(()(", "3"),
		testcase.New("example3", "))(((((", "3"),
		testcase.New("example4_1", "())", "-1"),
		testcase.New("example4_2", "))(", "-1"),
		testcase.New("example5_1", ")))", "-3"),
		testcase.New("example5_2", ")())())", "-3"),
		testcase.NewFile(inputFile, inputFile, "232"),
	} {
		tc.Test(t, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tcPart1.Benchmark(b, Part1)
}

func TestPart2(t *testing.T) {
	for _, tc := range []testcase.TestCase2{
		testcase.New("example1", ")", "1"),
		testcase.New("example2", "()())", "5"),
		testcase.NewFile(inputFile, inputFile, "1783"),
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
