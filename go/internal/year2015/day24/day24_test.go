package day24

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const (
	exampleFile = "testdata/example"
	inputFile   = "../testdata/24"
)

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.FromFile(t, exampleFile, "99"),
		testcase.FromFile(t, inputFile, ""),
	} {
		testcase.Run(t, tc, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part1)
}

// func TestPart2(t *testing.T) {
// 	tc := testcase.FromFile(t, inputFile, "")
// 	testcase.Run(t, tc, Part2)
// }

// func BenchmarkPart2(b *testing.B) {
// 	tc := testcase.FromFile(b, inputFile, "")
// 	testcase.Bench(b, tc, Part2)
// }
