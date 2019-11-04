package day06

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

func TestDay06(t *testing.T) {
	t.Run("part1", func(t *testing.T) {
		for _, tc := range []testcase.TestCase{
			testcase.FromString("example1", "turn on 0,0 through 999,999", "1000000"),
			testcase.FromString("example2", "toggle 0,0 through 999,0", "1000"),
			testcase.FromString("example3", "turn off 499,499 through 500,500", "0"),
			testcase.FromInputFile(t, 2015, 6, "569999"),
		} {
			testcase.Run(t, tc, Day06One)
		}
	})
	t.Run("part2", func(t *testing.T) {
		for _, tc := range []testcase.TestCase{
			testcase.FromString("example1", "turn on 0,0 through 0,0", "1"),
			testcase.FromString("example2", "toggle 0,0 through 999,999", "2000000"),
			testcase.FromInputFile(t, 2015, 6, "17836115"),
		} {
			testcase.Run(t, tc, Day06Two)
		}
	})
}

func BenchmarkDay06(b *testing.B) {
	tc := testcase.FromInputFile(b, 2015, 6, "")
	b.Run("part1", func(b *testing.B) {
		testcase.Bench(b, tc, Day06One)
	})
	b.Run("part2", func(b *testing.B) {
		testcase.Bench(b, tc, Day06Two)
	})
}
