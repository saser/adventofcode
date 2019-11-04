package day02

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

func TestDay02(t *testing.T) {
	t.Run("part1", func(t *testing.T) {
		for _, tc := range []testcase.TestCase{
			testcase.FromString("example1", "2x3x4", "58"),
			testcase.FromString("example2", "1x1x10", "43"),
			testcase.FromInputFile(t, 2015, 2, "1586300"),
		} {
			testcase.Run(t, tc, Day02One)
		}
	})
	t.Run("part2", func(t *testing.T) {
		for _, tc := range []testcase.TestCase{
			testcase.FromString("example1", "2x3x4", "34"),
			testcase.FromString("example2", "1x1x10", "14"),
			testcase.FromInputFile(t, 2015, 2, "3737498"),
		} {
			testcase.Run(t, tc, Day02Two)
		}
	})
}

func BenchmarkDay02(b *testing.B) {
	tc := testcase.FromInputFile(b, 2015, 2, "")
	b.Run("part1", func(b *testing.B) {
		testcase.Bench(b, tc, Day01One)
	})
	b.Run("part2", func(b *testing.B) {
		testcase.Bench(b, tc, Day01Two)
	})
}
