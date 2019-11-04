package year2015

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

func TestDay01(t *testing.T) {
	t.Run("part1", func(t *testing.T) {
		for _, tc := range []testcase.TestCase{
			testcase.FromString("example1_1", "(())", "0"),
			testcase.FromString("example1_2", "()()", "0"),
			testcase.FromString("example2_1", "(((", "3"),
			testcase.FromString("example2_2", "(()(()(", "3"),
			testcase.FromString("example3", "))(((((", "3"),
			testcase.FromString("example4_1", "())", "-1"),
			testcase.FromString("example4_2", "))(", "-1"),
			testcase.FromString("example5_1", ")))", "-3"),
			testcase.FromString("example5_2", ")())())", "-3"),
			testcase.FromInputFile(t, 2015, 1, "232"),
		} {
			testcase.Run(t, tc, Day01One)
		}
	})
	t.Run("part2", func(t *testing.T) {
		for _, tc := range []testcase.TestCase{
			testcase.FromString("example1", ")", "1"),
			testcase.FromString("example2", "()())", "5"),
			testcase.FromInputFile(t, 2015, 1, "1783"),
		} {
			testcase.Run(t, tc, Day01Two)
		}
	})
}

func BenchmarkDay01(b *testing.B) {
	tc := testcase.FromInputFile(b, 2015, 1, "")
	b.Run("part1", func(b *testing.B) {
		testcase.Bench(b, tc, Day01One)
	})
	b.Run("part2", func(b *testing.B) {
		testcase.Bench(b, tc, Day01Two)
	})
}
