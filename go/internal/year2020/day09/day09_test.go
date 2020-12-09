package day09

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const (
	inputFile       = "../testdata/09"
	defaultLookback = 25

	exampleFile     = "testdata/example"
	exampleLookback = 5
)

var (
	tcPart1 = testcase.NewFile("input", inputFile, "27911108")
	tcPart2 = testcase.NewFile("input", inputFile, "4023754")
)

func TestPart1(t *testing.T) {
	for _, tt := range []struct {
		tc       testcase.TestCase
		lookback int
	}{
		{tc: testcase.NewFile("example", exampleFile, "127"), lookback: exampleLookback},
		{tc: tcPart1, lookback: defaultLookback},
	} {
		reset := Lookback
		Lookback = tt.lookback
		tt.tc.Test(t, Part1)
		Lookback = reset
	}
}

func BenchmarkPart1(b *testing.B) {
	tcPart1.Benchmark(b, Part1)
}

func TestPart2(t *testing.T) {
	for _, tt := range []struct {
		tc       testcase.TestCase
		lookback int
	}{
		{tc: testcase.NewFile("example", exampleFile, "62"), lookback: exampleLookback},
		{tc: tcPart2, lookback: defaultLookback},
	} {
		reset := Lookback
		Lookback = tt.lookback
		tt.tc.Test(t, Part2)
		Lookback = reset
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
