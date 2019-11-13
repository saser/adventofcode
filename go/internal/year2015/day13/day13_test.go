package day13

import (
	"os"
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
	"github.com/stretchr/testify/require"
)

const (
	exampleFile = "testdata/example"
	inputFile   = "../testdata/13"
)

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

func Test_parse(t *testing.T) {
	file, err := os.Open(exampleFile)
	require.NoError(t, err)
	m, err := parse(file)
	require.NoError(t, err)
	expected := map[string]map[string]int{
		"Alice": map[string]int{
			"Bob":   54,
			"Carol": -79,
			"David": -2,
		},
		"Bob": map[string]int{
			"Alice": 83,
			"Carol": -7,
			"David": -63,
		},
		"Carol": map[string]int{
			"Alice": -62,
			"Bob":   60,
			"David": 55,
		},
		"David": map[string]int{
			"Alice": 46,
			"Bob":   -7,
			"Carol": 41,
		},
	}
	require.Equal(t, expected, m)
}

func Test_score(t *testing.T) {
	file, err := os.Open(exampleFile)
	require.NoError(t, err)
	m, err := parse(file)
	require.NoError(t, err)
	names := []string{"Alice", "Bob", "Carol", "David"}
	require.Equal(t, 330, score(names, m))
}

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.FromFile(t, exampleFile, "330"),
		testcase.FromFile(t, inputFile, "618"),
	} {
		testcase.Run(t, tc, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part1)
}
