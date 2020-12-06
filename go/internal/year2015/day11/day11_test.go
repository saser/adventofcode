package day11

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/11"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "cqjxxyzz")
	tcPart2 = testcase.NewFile("input", inputFile, "cqkaabcc")
)

func intsEqual(x, y []int) bool {
	if len(x) != len(y) {
		return false
	}
	for i, xn := range x {
		if y[i] != xn {
			return false
		}
	}
	return true
}

func Test_digitsToInts(t *testing.T) {
	for _, tt := range []struct {
		s    string
		want []int
	}{
		{s: "a", want: []int{0}},
		{s: "z", want: []int{25}},
		{s: "aa", want: []int{0, 0}},
		{s: "abc", want: []int{0, 1, 2}},
	} {
		if got := digitsToInts(tt.s); !intsEqual(got, tt.want) {
			t.Errorf("digitsToInts(%q) = %v; want %v", tt.s, got, tt.want)
		}
	}
}

func Test_intsToDigits(t *testing.T) {
	for _, tt := range []struct {
		is   []int
		want string
	}{
		{is: []int{0}, want: "a"},
		{is: []int{25}, want: "z"},
		{is: []int{0, 0}, want: "aa"},
		{is: []int{0, 1, 2}, want: "abc"},
	} {
		if got := intsToDigits(tt.is); got != tt.want {
			t.Errorf("intsToDigits(%v) = %q; want %q", tt.is, got, tt.want)
		}
	}
}

func Test_next(t *testing.T) {
	for _, tt := range []struct {
		s    string
		want string
	}{
		{s: "a", want: "b"},
		{s: "z", want: "aa"},
		{s: "aa", want: "ab"},
		{s: "zy", want: "zz"},
		{s: "zz", want: "aaa"},
	} {
		if got := next(tt.s); got != tt.want {
			t.Errorf("next(%q) = %q; want %q", tt.s, got, tt.want)
		}
	}
}

func Test_hasIncreasing(t *testing.T) {
	for _, tt := range []struct {
		s    string
		want bool
	}{
		{s: "hijklmmn", want: true},
		{s: "abbceffg", want: false},
		{s: "abbcegjk", want: false},
		{s: "abcdffaa", want: true},
		{s: "ghjaabcc", want: true},
	} {
		if got := hasIncreasing(tt.s); got != tt.want {
			t.Errorf("hasIncreasing(%q) = %v; want %v", tt.s, got, tt.want)
		}
	}
}

func Test_hasNoIOL(t *testing.T) {
	for _, tt := range []struct {
		s    string
		want bool
	}{
		{s: "hijklmmn", want: false},
		{s: "abbceffg", want: true},
		{s: "abbcegjk", want: true},
		{s: "abcdffaa", want: true},
		{s: "ghjaabcc", want: true},
	} {
		if got := hasNoIOL(tt.s); got != tt.want {
			t.Errorf("hasNoIOL(%q) = %v; want %v", tt.s, got, tt.want)
		}
	}
}

func Test_hasTwoPairs(t *testing.T) {
	for _, tt := range []struct {
		s    string
		want bool
	}{
		{s: "hijklmmn", want: false},
		{s: "abbceffg", want: true},
		{s: "abbcegjk", want: false},
		{s: "abcdffaa", want: true},
		{s: "ghjaabcc", want: true},
	} {
		if got := hasTwoPairs(tt.s); got != tt.want {
			t.Errorf("hasTwoPairs(%q) = %v; want %v", tt.s, got, tt.want)
		}
	}
}

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.New("example1", "abcdefgh", "abcdffaa"),
		testcase.New("example2", "ghijklmn", "ghjaabcc"),
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
