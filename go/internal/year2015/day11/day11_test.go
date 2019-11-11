package day11

import (
	"fmt"
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
	"github.com/stretchr/testify/require"
)

const inputFile = "../testdata/11"

func Test_hasIncreasing(t *testing.T) {
	for _, tt := range []struct {
		s string
		b bool
	}{
		{s: "hijklmmn", b: true},
		{s: "abbceffg", b: false},
		{s: "abbcegjk", b: false},
		{s: "abcdffaa", b: true},
		{s: "ghjaabcc", b: true},
	} {
		t.Run(fmt.Sprintf("s=%v", tt.s), func(t *testing.T) {
			require.Equal(t, tt.b, hasIncreasing(tt.s))
		})
	}
}

func Test_hasNoIOL(t *testing.T) {
	for _, tt := range []struct {
		s string
		b bool
	}{
		{s: "hijklmmn", b: false},
		{s: "abbceffg", b: true},
		{s: "abbcegjk", b: true},
		{s: "abcdffaa", b: true},
		{s: "ghjaabcc", b: true},
	} {
		t.Run(fmt.Sprintf("s=%v", tt.s), func(t *testing.T) {
			require.Equal(t, tt.b, hasNoIOL(tt.s))
		})
	}
}

func Test_hasTwoPairs(t *testing.T) {
	for _, tt := range []struct {
		s string
		b bool
	}{
		{s: "hijklmmn", b: false},
		{s: "abbceffg", b: true},
		{s: "abbcegjk", b: false},
		{s: "abcdffaa", b: true},
		{s: "ghjaabcc", b: true},
	} {
		t.Run(fmt.Sprintf("s=%v", tt.s), func(t *testing.T) {
			require.Equal(t, tt.b, hasTwoPairs(tt.s))
		})
	}
}

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.FromString("example1", "abcdefgh", "abcdffaa"),
		testcase.FromString("example2", "ghijklmn", "ghjaabcc"),
	} {
		testcase.Run(t, tc, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part1)
}
