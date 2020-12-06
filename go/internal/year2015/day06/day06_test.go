package day06

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/06"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "569999")
	tcPart2 = testcase.NewFile("input", inputFile, "17836115")
)

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase2{
		testcase.New("example1", "turn on 0,0 through 999,999", "1000000"),
		testcase.New("example2", "toggle 0,0 through 999,0", "1000"),
		testcase.New("example3", "turn off 499,499 through 500,500", "0"),
		tcPart1,
	} {
		tc.Test(t, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tcPart1.Benchmark(b, Part1)
}

func TestPart2(t *testing.T) {
	for _, tc := range []testcase.TestCase2{
		testcase.New("example1", "turn on 0,0 through 0,0", "1"),
		testcase.New("example2", "toggle 0,0 through 999,999", "2000000"),
		tcPart2,
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
