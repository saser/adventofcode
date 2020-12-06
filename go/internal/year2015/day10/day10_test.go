package day10

import (
	"fmt"
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
	"github.com/stretchr/testify/require"
)

const inputFile = "../testdata/10"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "492982")
	tcPart2 = testcase.NewFile("input", inputFile, "6989950")
)

func Test_lookAndSay(t *testing.T) {
	for _, tt := range []struct {
		in  string
		out string
	}{
		{in: "1", out: "11"},
		{in: "11", out: "21"},
		{in: "21", out: "1211"},
		{in: "1211", out: "111221"},
		{in: "111221", out: "312211"},
	} {
		t.Run(fmt.Sprintf("in=%v", tt.in), func(t *testing.T) {
			require.Equal(t, tt.out, lookAndSay(tt.in))
		})
	}
}

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
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
		tcPart2,
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
