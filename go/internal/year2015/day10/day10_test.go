package day10

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/10"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "492982")
	tcPart2 = testcase.NewFile("input", inputFile, "6989950")
)

func Test_lookAndSay(t *testing.T) {
	for _, tt := range []struct {
		in   string
		want string
	}{
		{in: "1", want: "11"},
		{in: "11", want: "21"},
		{in: "21", want: "1211"},
		{in: "1211", want: "111221"},
		{in: "111221", want: "312211"},
	} {
		if got := lookAndSay(tt.in); got != tt.want {
			t.Errorf("lookAndSay(%q) = %q; want %q", tt.in, got, tt.want)
		}
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
