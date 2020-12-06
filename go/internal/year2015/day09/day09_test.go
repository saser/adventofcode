package day09

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/09"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "")
	tcPart2 = testcase.NewFile("input", inputFile, "")
)

func TestPart1(t *testing.T) {
	example := `London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141`
	for _, tc := range []testcase.TestCase2{
		testcase.New("example", example, "605"),
		testcase.NewFile(inputFile, inputFile, "251"),
	} {
		tc.Test(t, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tcPart1.Benchmark(b, Part1)
}

func TestPart2(t *testing.T) {
	example := `London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141`
	for _, tc := range []testcase.TestCase2{
		testcase.New("example", example, "982"),
		testcase.NewFile(inputFile, inputFile, "898"),
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
