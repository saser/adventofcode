package day04

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

func TestDay04(t *testing.T) {
	t.Run("part1", func(t *testing.T) {
		for _, tc := range []testcase.TestCase{
			testcase.FromString("example1", "abcdef", "609043"),
			testcase.FromString("example2", "pqrstuv", "1048970"),
			testcase.FromInputFile(t, 2015, 4, "346386"),
		} {
			testcase.Run(t, tc, Day04One)
		}
	})
	t.Run("part2", func(t *testing.T) {
		for _, tc := range []testcase.TestCase{
			testcase.FromInputFile(t, 2015, 4, "9958218"),
		} {
			testcase.Run(t, tc, Day04Two)
		}
	})
}

func BenchmarkDay04(b *testing.B) {
	tc := testcase.FromInputFile(b, 2015, 4, "")
	b.Run("part1", func(b *testing.B) {
		testcase.Bench(b, tc, Day04One)
	})
	b.Run("part2", func(b *testing.B) {
		testcase.Bench(b, tc, Day04Two)
	})
}
