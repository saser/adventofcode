package year2015

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

func TestDay03(t *testing.T) {
	t.Run("part1", func(t *testing.T) {
		for _, tc := range []testcase.TestCase{
			testcase.FromString("example1", ">", "2"),
			testcase.FromString("example2", "^>v<", "4"),
			testcase.FromString("example3", "^v^v^v^v^v", "2"),
			testcase.FromInputFile(t, 2015, 3, "2572"),
		} {
			testcase.Run(t, tc, Day03One)
		}
	})
}

func BenchmarkDay03(b *testing.B) {
	tc := testcase.FromInputFile(b, 2015, 3, "")
	b.Run("part1", func(b *testing.B) {
		testcase.Bench(b, tc, Day03One)
	})
}
