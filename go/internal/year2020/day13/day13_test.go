package day13

import (
	"math/big"
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

func Test_crt(t *testing.T) {
	for _, tt := range []struct {
		eqs  []eq
		want *big.Int
	}{
		{
			eqs: []eq{
				{rem: big.NewInt(0), mod: big.NewInt(3)},
				{rem: big.NewInt(3), mod: big.NewInt(4)},
				{rem: big.NewInt(4), mod: big.NewInt(5)},
			},
			want: big.NewInt(39),
		},
	} {
		if got := crt(tt.eqs); got.Cmp(tt.want) != 0 {
			t.Errorf("crt(%+v) = %v; want %v", tt.eqs, got, tt.want)
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
