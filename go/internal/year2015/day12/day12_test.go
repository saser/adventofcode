package day12

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/12"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "111754")
	tcPart2 = testcase.NewFile("input", inputFile, "65402")
)

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase2{
		testcase.New("example1_1", `[1,2,3]`, "6"),
		testcase.New("example1_2", `{"a":2,"b":4}`, "6"),
		testcase.New("example2_1", `[[[3]]]`, "3"),
		testcase.New("example2_2", `{"a":{"b":4},"c":-1}`, "3"),
		testcase.New("example3_1", `{"a":[-1,1]}`, "0"),
		testcase.New("example3_2", `[-1,{"a":1}]`, "0"),
		testcase.New("example4_1", `[]`, "0"),
		testcase.New("example4_2", `{}`, "0"),
		tcPart1,
	} {
		tc.Test(t, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tcPart1.Benchmark(b, Part1)
}

func TestPart2(t *testing.T) {
	for _, tc := range []testcase.TestCase2{
		testcase.New("example1", `[1,2,3]`, "6"),
		testcase.New("example2", `[1,{"c":"red","b":2},3]`, "4"),
		testcase.New("example3", `{"d":"red","e":[1,2,3,4],"f":5}`, "0"),
		testcase.New("example4", `[1,"red",5]`, "6"),
		tcPart2,
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
