package day23

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/23"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "307")
	tcPart2 = testcase.NewFile("input", inputFile, "160")
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
