package day09

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/09"

func TestPart1(t *testing.T) {
	example := `London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141`
	for _, tc := range []testcase.TestCase{
		testcase.FromString("example", example, "605"),
		testcase.FromFile(t, inputFile, "251"),
	} {
		testcase.Run(t, tc, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part1)
}

func TestPart2(t *testing.T) {
	example := `London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141`
	for _, tc := range []testcase.TestCase{
		testcase.FromString("example", example, "982"),
	} {
		testcase.Run(t, tc, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part2)
}
