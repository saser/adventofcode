package day12

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/12"

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.FromString("example1_1", `[1,2,3]`, "6"),
		testcase.FromString("example1_2", `{"a":2,"b":4}`, "6"),
		testcase.FromString("example2_1", `[[[3]]]`, "3"),
		testcase.FromString("example2_2", `{"a":{"b":4},"c":-1}`, "3"),
		testcase.FromString("example3_1", `{"a":[-1,1]}`, "0"),
		testcase.FromString("example3_2", `[-1,{"a":1}]`, "0"),
		testcase.FromString("example4_1", `[]`, "0"),
		testcase.FromString("example4_2", `{}`, "0"),
		testcase.FromFile(t, inputFile, "111754"),
	} {
		testcase.Run(t, tc, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part1)
}

func TestPart2(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.FromString("example1", `[1,2,3]`, "6"),
		testcase.FromString("example2", `[1,{"c":"red","b":2},3]`, "4"),
		testcase.FromString("example3", `{"d":"red","e":[1,2,3,4],"f":5}`, "0"),
		testcase.FromString("example4", `[1,"red",5]`, "6"),
	} {
		testcase.Run(t, tc, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part2)
}
