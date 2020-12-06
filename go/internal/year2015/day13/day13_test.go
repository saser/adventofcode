package day13

import (
	"io/ioutil"
	"reflect"
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const (
	exampleFile = "testdata/example"
	inputFile   = "../testdata/13"
)

var (
	tcPart1 = testcase.NewFile("input", inputFile, "618")
	tcPart2 = testcase.NewFile("input", inputFile, "601")
)

func Test_parsePreference(t *testing.T) {
	for _, tt := range []struct {
		s    string
		want preference
	}{
		{
			s: "Alice would gain 54 happiness units by sitting next to Bob.",
			want: preference{
				from:   "Alice",
				to:     "Bob",
				change: 54,
			},
		},
		{
			s: "Alice would gain 79 happiness units by sitting next to Carol.",
			want: preference{
				from:   "Alice",
				to:     "Carol",
				change: 79,
			},
		},
	} {
		got, err := parsePreference(tt.s)
		if err != nil {
			t.Errorf("parsePreference(%q) err = %v", tt.s, err)
		}
		if got != tt.want {
			t.Errorf("parsePreference(%q) preference = %v; want %v", tt.s, got, tt.want)
		}
	}
}

func Test_parse(t *testing.T) {
	data, err := ioutil.ReadFile(exampleFile)
	if err != nil {
		t.Fatal(err)
	}
	got, err := parse(string(data))
	if err != nil {
		t.Fatal(err)
	}
	want := map[string]map[string]int{
		"Alice": {
			"Bob":   54,
			"Carol": -79,
			"David": -2,
		},
		"Bob": {
			"Alice": 83,
			"Carol": -7,
			"David": -63,
		},
		"Carol": {
			"Alice": -62,
			"Bob":   60,
			"David": 55,
		},
		"David": {
			"Alice": 46,
			"Bob":   -7,
			"Carol": 41,
		},
	}
	if !reflect.DeepEqual(got, want) {
		t.Errorf("parse(...) got = %v; want %v", got, want)
	}
}

func Test_score(t *testing.T) {
	data, err := ioutil.ReadFile(exampleFile)
	if err != nil {
		t.Fatal(err)
	}
	m, err := parse(string(data))
	if err != nil {
		t.Fatal(err)
	}
	names := []string{"Alice", "Bob", "Carol", "David"}
	if got, want := score(names, m, 1), 330; got != want {
		t.Errorf("scores(%v, %v, %v) = %v; want %v", names, m, 1, got, want)
	}
}

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.NewFile(exampleFile, exampleFile, "330"),
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
