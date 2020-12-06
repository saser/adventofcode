package day18

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const (
	exampleFile            = "testdata/example"
	exampleIterationsPart1 = 4
	exampleIterationsPart2 = 5
	exampleGridSize        = 6
	inputFile              = "../testdata/18"
)

func TestPart1(t *testing.T) {
	for _, tt := range []struct {
		tc         testcase.TestCase2
		iterations int
		gridSize   int
	}{
		{tc: testcase.NewFile(exampleFile, exampleFile, "4"), iterations: exampleIterationsPart1, gridSize: exampleGridSize},
		{tc: testcase.NewFile(inputFile, inputFile, "1061"), iterations: Iterations, gridSize: GridSize},
	} {
		tt.tc.Test(t, Part1(tt.iterations, tt.gridSize))
	}
}

func BenchmarkPart1(b *testing.B) {
	testcase.Bench(b, tc, Part1(Iterations, GridSize))
}

func TestPart2(t *testing.T) {
	for _, tt := range []struct {
		tc         testcase.TestCase2
		iterations int
		gridSize   int
	}{
		{tc: testcase.NewFile(exampleFile, exampleFile, "17"), iterations: exampleIterationsPart2, gridSize: exampleGridSize},
		{tc: testcase.NewFile(inputFile, inputFile, "1006"), iterations: Iterations, gridSize: GridSize},
	} {
		tt.tc.Test(t, Part2(tt.iterations, tt.gridSize))
	}
}

func BenchmarkPart2(b *testing.B) {
	testcase.Bench(b, tc, Part2(Iterations, GridSize))
}
