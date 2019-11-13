package day13

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
	"github.com/stretchr/testify/require"
)

const inputFile = "../testdata/13"

func Test_parsePreference(t *testing.T) {
	for _, tt := range []struct {
		name string
		s    string
		p    preference
	}{
		{
			name: "AliceBobGain54",
			s:    "Alice would gain 54 happiness units by sitting next to Bob.",
			p: preference{
				from:   "Alice",
				to:     "Bob",
				change: 54,
			},
		},
		{
			name: "AliceCarolLose79",
			s:    "Alice would gain 79 happiness units by sitting next to Carol.",
			p: preference{
				from:   "Alice",
				to:     "Carol",
				change: 79,
			},
		},
	} {
		t.Run(tt.name, func(t *testing.T) {
			p, err := parsePreference(tt.s)
			require.NoError(t, err)
			require.Equal(t, tt.p, p)
		})
	}
}

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.FromFile(t, "testdata/example", "330"),
	} {
		testcase.Run(t, tc, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part1)
}
