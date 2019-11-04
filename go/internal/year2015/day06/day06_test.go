package day06

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/06"

func TestDay06(t *testing.T) {
	t.Run("part1", func(t *testing.T) {
		for _, tc := range []testcase.TestCase{
			testcase.FromString("example1", "turn on 0,0 through 999,999", "1000000"),
			testcase.FromString("example2", "toggle 0,0 through 999,0", "1000"),
			testcase.FromString("example3", "turn off 499,499 through 500,500", "0"),
			testcase.FromFile(t, inputFile, "569999"),
		} {
			testcase.Run(t, tc, Part1)
		}
	})
	t.Run("part2", func(t *testing.T) {
		for _, tc := range []testcase.TestCase{
			testcase.FromString("example1", "turn on 0,0 through 0,0", "1"),
			testcase.FromString("example2", "toggle 0,0 through 999,999", "2000000"),
			testcase.FromFile(t, inputFile, "17836115"),
		} {
			testcase.Run(t, tc, Part2)
		}
	})
}

func BenchmarkDay06(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	b.Run("part1", func(b *testing.B) {
		testcase.Bench(b, tc, Part1)
	})
	b.Run("part2", func(b *testing.B) {
		testcase.Bench(b, tc, Part2)
	})
}
