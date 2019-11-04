package day03

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/03"

func TestDay03(t *testing.T) {
	t.Run("part1", func(t *testing.T) {
		for _, tc := range []testcase.TestCase{
			testcase.FromString("example1", ">", "2"),
			testcase.FromString("example2", "^>v<", "4"),
			testcase.FromString("example3", "^v^v^v^v^v", "2"),
			testcase.FromFile(t, inputFile, "2572"),
		} {
			testcase.Run(t, tc, Part1)
		}
	})
	t.Run("part2", func(t *testing.T) {
		for _, tc := range []testcase.TestCase{
			testcase.FromString("example1", "^v", "3"),
			testcase.FromString("example2", "^>v<", "3"),
			testcase.FromString("example3", "^v^v^v^v^v", "11"),
			testcase.FromFile(t, inputFile, "2631"),
		} {
			testcase.Run(t, tc, Part2)
		}
	})
}

func BenchmarkDay03(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	b.Run("part1", func(b *testing.B) {
		testcase.Bench(b, tc, Part1)
	})
	b.Run("part2", func(b *testing.B) {
		testcase.Bench(b, tc, Part2)
	})
}
