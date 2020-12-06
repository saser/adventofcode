package day16

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/16"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "103")
	tcPart2 = testcase.NewFile("input", inputFile, "405")
)

func TestPart1(t *testing.T) {
	tcPart1.Test(t, Part1)
}

func BenchmarkPart1(b *testing.B) {
	tcPart1.Benchmark(b, Part1)
}

func TestPart2(t *testing.T) {
	tcPart2.Test(t, Part2)
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
