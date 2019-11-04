package day02

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/02"

func TestDay02(t *testing.T) {
	t.Run("part1", func(t *testing.T) {
		for _, tc := range []testcase.TestCase{
			testcase.FromString("example1", "2x3x4", "58"),
			testcase.FromString("example2", "1x1x10", "43"),
			testcase.FromFile(t, inputFile, "1586300"),
		} {
			testcase.Run(t, tc, Part1)
		}
	})
	t.Run("part2", func(t *testing.T) {
		for _, tc := range []testcase.TestCase{
			testcase.FromString("example1", "2x3x4", "34"),
			testcase.FromString("example2", "1x1x10", "14"),
			testcase.FromFile(t, inputFile, "3737498"),
		} {
			testcase.Run(t, tc, Part2)
		}
	})
}

func BenchmarkDay02(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	b.Run("part1", func(b *testing.B) {
		testcase.Bench(b, tc, Part1)
	})
	b.Run("part2", func(b *testing.B) {
		testcase.Bench(b, tc, Part2)
	})
}
