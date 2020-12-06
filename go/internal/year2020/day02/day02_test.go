package day02

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/02"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "396")
	tcPart2 = testcase.NewFile("input", inputFile, "428")
)

func Test_parsers(t *testing.T) {
	for _, tt := range []struct {
		s    string
		want entry
	}{
		{
			s: "1-3 a: abcde",
			want: entry{
				Low:      1,
				High:     3,
				Letter:   'a',
				Password: "abcde",
			},
		},
		{
			s: "1-3 b: cdefg",
			want: entry{
				Low:      1,
				High:     3,
				Letter:   'b',
				Password: "cdefg",
			},
		},
		{
			s: "2-9 c: ccccccccc",
			want: entry{
				Low:      2,
				High:     9,
				Letter:   'c',
				Password: "ccccccccc",
			},
		},
	} {
		{
			got, err := parseRegexp(tt.s)
			if err != nil {
				t.Errorf("parseRegexp(%q) err = %v", tt.s, err)
			}
			if got != tt.want {
				t.Errorf("parseRegexp(%q) entry = %v; want %v", tt.s, got, tt.want)
			}
		}
		{
			got, err := parseManual(tt.s)
			if err != nil {
				t.Errorf("parseManual(%q) err = %v", tt.s, err)
			}
			if got != tt.want {
				t.Errorf("parseManual(%q) entry = %v; want %v", tt.s, got, tt.want)
			}
		}
	}
}

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase2{
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
		tcPart2,
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
