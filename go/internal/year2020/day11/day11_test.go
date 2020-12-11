package day11

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const (
	inputFile   = "../testdata/11"
	exampleFile = "testdata/example"
)

var (
	tcPart1 = testcase.NewFile("input", inputFile, "2263")
	tcPart2 = testcase.NewFile("input", inputFile, "2002")
)

func Test_visibleCounter_count(t *testing.T) {
	for _, tt := range []struct {
		input    string
		row, col int
		want     int
	}{
		{
			input: `.......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....`,
			row: 4, col: 3,
			want: 8,
		},
		{
			input: `.............
.L.L.#.#.#.#.
.............`,
			row: 1, col: 1,
			want: 0,
		},
		{
			input: `.##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.`,
			row: 3, col: 3,
			want: 0,
		},
	} {
		g := newGrid(tt.input)
		c := visibleCounter{}
		if got := c.count(g, tt.row, tt.col); got != tt.want {
			t.Errorf("c.count(%v, %v) = %v; want %v\n%s", tt.row, tt.col, got, tt.want, tt.input)
		}
	}
}

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.NewFile("example", exampleFile, "37"),
		tcPart1,
	} {
		tc.Test(t, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tcPart1.Benchmark(b, Part1)
}

func TestPart2(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.NewFile("example", exampleFile, "26"),
		tcPart2,
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
