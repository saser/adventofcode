package year2015

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

func TestDay05(t *testing.T) {
	t.Run("part1", func(t *testing.T) {
		for _, tc := range []testcase.TestCase{
			testcase.FromString("example1", "ugknbfddgicrmopn", "1"),
			testcase.FromString("example2", "aaa", "1"),
			testcase.FromString("example3", "jchzalrnumimnmhp", "0"),
			testcase.FromString("example4", "haegwjzuvuyypxyu", "0"),
			testcase.FromString("example5", "dvszwmarrgswjxmb", "0"),
			testcase.FromInputFile(t, 2015, 5, "255"),
		} {
			testcase.Run(t, tc, Day05One)
		}
	})
}

func BenchmarkDay05(b *testing.B) {
	tc := testcase.FromInputFile(b, 2015, 5, "")
	b.Run("part1", func(b *testing.B) {
		testcase.Bench(b, tc, Day05One)
	})
}
