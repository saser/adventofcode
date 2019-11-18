package day17

import (
	"testing"

	"github.com/Saser/adventofcode/internal/solution"
	"github.com/Saser/adventofcode/internal/testcase"
)

const (
	exampleFile   = "testdata/example"
	exampleTarget = 25
	inputFile     = "../testdata/17"
)

func TestPart1(t *testing.T) {
	for _, tt := range []struct {
		tc  testcase.TestCase
		sol solution.Solution
	}{
		{tc: testcase.FromFile(t, exampleFile, "4"), sol: Part1(exampleTarget)},
		{tc: testcase.FromFile(t, inputFile, "1304"), sol: Part1(Target)},
	} {
		testcase.Run(t, tt.tc, tt.sol)
	}
}

func BenchmarkPart1(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part1(Target))
}

func TestPart2(t *testing.T) {
	for _, tt := range []struct {
		tc  testcase.TestCase
		sol solution.Solution
	}{
		{tc: testcase.FromFile(t, exampleFile, "3"), sol: Part2(exampleTarget)},
		{tc: testcase.FromFile(t, inputFile, "18"), sol: Part2(Target)},
	} {
		testcase.Run(t, tt.tc, tt.sol)
	}
}

func BenchmarkPart2(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part2(Target))
}
