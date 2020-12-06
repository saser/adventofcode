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

var (
	tcPart1 = testcase.NewFile("input", inputFile, "1304")
	tcPart2 = testcase.NewFile("input", inputFile, "18")
)

func TestPart1(t *testing.T) {
	for _, tt := range []struct {
		tc     testcase.TestCase2
		target int
	}{
		{tc: testcase.NewFile("example", exampleFile, "4"), target: exampleTarget},
		{tc: tcPart1, target: Target},
	} {
		reset := Target
		Target = tt.target
		tt.tc.Test(t, Part1)
		Target = reset
	}
}

func BenchmarkPart1(b *testing.B) {
	tcPart1.Benchmark(b, Part1)
}

func TestPart2(t *testing.T) {
	for _, tt := range []struct {
		tc     testcase.TestCase2
		target int
	}{
		{tc: testcase.NewFile("example", exampleFile, "3"), target: exampleTarget},
		{tc: tcPart2, target: Target},
	} {
		reset := Target
		Target = tt.target
		tt.tc.Test(t, Part2)
		Target = reset
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
