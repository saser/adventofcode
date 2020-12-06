package day25

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/25"

var tcPart1 = testcase.NewFile("input", inputFile, "19980801")

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase2{
		testcase.New("example", "To continue, please consult the code grid in the manual.  Enter the code at row 2, column 1.", "31916031"),
		tcPart1,
	} {
		tc.Test(t, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tcPart1.Benchmark(b, Part1)
}
