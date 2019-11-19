package day18

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const (
	exampleFile       = "testdata/example"
	exampleIterations = 4
	exampleGridSize   = 6
	inputFile         = "../testdata/18"
)

func TestPart1(t *testing.T) {
	for _, tt := range []struct {
		tc         testcase.TestCase
		iterations int
		gridSize   int
	}{
		{tc: testcase.FromFile(t, exampleFile, "4"), iterations: exampleIterations, gridSize: exampleGridSize},
		{tc: testcase.FromFile(t, inputFile, ""), iterations: Iterations, gridSize: GridSize},
	} {
		testcase.Run(t, tt.tc, Part1(tt.iterations, tt.gridSize))
	}
}

func BenchmarkPart1(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part1(Iterations, GridSize))
}
