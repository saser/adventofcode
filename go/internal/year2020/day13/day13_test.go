package day13

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const (
	inputFile   = "../testdata/13"
	exampleFile = "testdata/example"
)

var (
	tcPart1 = testcase.NewFile("input", inputFile, "203")
	tcPart2 = testcase.NewFile("input", inputFile, "905694340256752")
)

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.NewFile("example", exampleFile, "295"),
		tcPart1,
	} {
		tc.Test(t, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tcPart1.Benchmark(b, Part1)
}

func Test_earliest(t *testing.T) {
	for _, tt := range []struct {
		buses []bus
		want  int
	}{
		{
			buses: []bus{
				{idx: 0, id: 7},
				{idx: 1, id: 13},
			},
			want: 77,
		},
		{
			buses: []bus{
				{idx: 0, id: 7},
				{idx: 1, id: 13},
				{idx: 4, id: 59},
			},
			want: 350,
		},
		{
			buses: []bus{
				{idx: 0, id: 7},
				{idx: 1, id: 13},
				{idx: 4, id: 59},
				{idx: 6, id: 31},
			},
			want: 70147,
		},
		{
			buses: []bus{
				{idx: 0, id: 17},
				{idx: 2, id: 13},
				{idx: 3, id: 19},
			},
			want: 3417,
		},
	} {
		if got := earliest(tt.buses); got != tt.want {
			t.Errorf("earliest(%+v) = %v; want %v", tt.buses, got, tt.want)
		}
	}
}

func TestPart2(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.New("example1", "0\n7,13,x,x,59,x,31,19", "1068781"),
		testcase.New("example2", "0\n17,x,13,19", "3417"),
		testcase.New("example3", "0\n67,7,59,61", "754018"),
		testcase.New("example4", "0\n67,x,7,59,61", "779210"),
		testcase.New("example5", "0\n67,7,x,59,61", "1261476"),
		testcase.New("example6", "0\n1789,37,47,1889", "1202161486"),
		tcPart2,
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
