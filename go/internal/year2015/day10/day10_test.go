package day10

import (
	"fmt"
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
	"github.com/stretchr/testify/require"
)

const inputFile = "../testdata/10"

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
		testcase.FromFile(t, inputFile, "492982"),
	} {
		testcase.Run(t, tc, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part1)
}
