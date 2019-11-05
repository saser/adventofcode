package day01

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/01"

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.FromString("example1_1", "(())", "0"),
		testcase.FromString("example1_2", "()()", "0"),
		testcase.FromString("example2_1", "(((", "3"),
		testcase.FromString("example2_2", "(()(()(", "3"),
		testcase.FromString("example3", "))(((((", "3"),
		testcase.FromString("example4_1", "())", "-1"),
		testcase.FromString("example4_2", "))(", "-1"),
		testcase.FromString("example5_1", ")))", "-3"),
		testcase.FromString("example5_2", ")())())", "-3"),
		testcase.FromFile(t, inputFile, "232"),
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
		testcase.FromString("example1", ")", "1"),
		testcase.FromString("example2", "()())", "5"),
		testcase.FromFile(t, inputFile, "1783"),
	} {
		testcase.Run(t, tc, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part2)
}
