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

var (
	tcPart1 = testcase.NewFile("input", inputFile, "1061")
	tcPart2 = testcase.NewFile("input", inputFile, "1006")
)

func TestPart1(t *testing.T) {
	for _, tt := range []struct {
		tc         testcase.TestCase2
		iterations int
		gridSize   int
	}{
		{tc: testcase.NewFile(exampleFile, exampleFile, "4"), iterations: exampleIterationsPart1, gridSize: exampleGridSize},
		{tc: tcPart1, iterations: Iterations, gridSize: GridSize},
	} {
		resetIterations := Iterations
		Iterations = tt.iterations
		resetGridSize := GridSize
		GridSize = tt.gridSize

		tt.tc.Test(t, Part1)

		GridSize = resetGridSize
		Iterations = resetIterations
	}
}

func BenchmarkPart1(b *testing.B) {
	tcPart1.Benchmark(b, Part1)
}

func TestPart2(t *testing.T) {
	for _, tt := range []struct {
		tc         testcase.TestCase2
		iterations int
		gridSize   int
	}{
		{tc: testcase.NewFile(exampleFile, exampleFile, "17"), iterations: exampleIterationsPart2, gridSize: exampleGridSize},
		{tc: tcPart2, iterations: Iterations, gridSize: GridSize},
	} {
		resetIterations := Iterations
		Iterations = tt.iterations
		resetGridSize := GridSize
		GridSize = tt.gridSize

		tt.tc.Test(t, Part2)

		GridSize = resetGridSize
		Iterations = resetIterations
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
