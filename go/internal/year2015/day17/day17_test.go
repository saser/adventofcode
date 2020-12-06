package day17

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const (
	exampleFile   = "testdata/example"
	exampleTarget = 25
	inputFile     = "../testdata/17"
)

func TestPart1(t *testing.T) {
	for _, tt := range []struct {
		tc  testcase.TestCase2
		sol testcase.Solution
	}{
		{tc: testcase.NewFile(exampleFile, exampleFile, "4"), sol: Part1(exampleTarget)},
		{tc: testcase.NewFile(inputFile, inputFile, "1304"), sol: Part1(Target)},
	} {
		tt.tc.Test(t, tt.sol)
	}
}

func BenchmarkPart1(b *testing.B) {
	testcase.Bench(b, tc, Part1(Target))
}

func TestPart2(t *testing.T) {
	for _, tt := range []struct {
		tc  testcase.TestCase2
		sol testcase.Solution
	}{
		{tc: testcase.NewFile(exampleFile, exampleFile, "3"), sol: Part2(exampleTarget)},
		{tc: testcase.NewFile(inputFile, inputFile, "18"), sol: Part2(Target)},
	} {
		tt.tc.Test(t, tt.sol)
	}
}

func BenchmarkPart2(b *testing.B) {
	testcase.Bench(b, tc, Part2(Target))
}
