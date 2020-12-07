package day07

import (
	"io/ioutil"
	"reflect"
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const (
	inputFile    = "../testdata/07"
	example1File = "testdata/example1"
	example2File = "testdata/example2"
)

var (
	tcPart1 = testcase.NewFile("input", inputFile, "112")
	tcPart2 = testcase.NewFile("input", inputFile, "6260")
)

func Test_parseLine(t *testing.T) {
	for _, tt := range []struct {
		line         string
		wantKey      string
		wantContents map[string]int
	}{
		{
			line:    "light red bags contain 1 bright white bag, 2 muted yellow bags.",
			wantKey: "light red",
			wantContents: map[string]int{
				"bright white": 1,
				"muted yellow": 2,
			},
		},
		{
			line:    "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
			wantKey: "dark orange",
			wantContents: map[string]int{
				"bright white": 3,
				"muted yellow": 4,
			},
		},
		{
			line:    "bright white bags contain 1 shiny gold bag.",
			wantKey: "bright white",
			wantContents: map[string]int{
				"shiny gold": 1,
			},
		},
		{
			line:    "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
			wantKey: "muted yellow",
			wantContents: map[string]int{
				"shiny gold": 2,
				"faded blue": 9,
			},
		},
		{
			line:    "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
			wantKey: "shiny gold",
			wantContents: map[string]int{
				"dark olive":   1,
				"vibrant plum": 2,
			},
		},
		{
			line:    "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
			wantKey: "dark olive",
			wantContents: map[string]int{
				"faded blue":   3,
				"dotted black": 4,
			},
		},
		{
			line:    "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
			wantKey: "vibrant plum",
			wantContents: map[string]int{
				"faded blue":   5,
				"dotted black": 6,
			},
		},
		{
			line:         "faded blue bags contain no other bags.",
			wantKey:      "faded blue",
			wantContents: nil,
		},
		{
			line:         "dotted black bags contain no other bags.",
			wantKey:      "dotted black",
			wantContents: nil,
		},
	} {
		gotKey, gotContents := parseLine(tt.line)
		if gotKey != tt.wantKey {
			t.Errorf("parseLine(%q) key = %v; want %v", tt.line, gotKey, tt.wantKey)
		}
		if !reflect.DeepEqual(gotContents, tt.wantContents) {
			t.Errorf("parseLine(%q) contents = %v; want %v", tt.line, gotContents, tt.wantContents)
		}
	}
}

func Test_parse(t *testing.T) {
	data, err := ioutil.ReadFile(example1File)
	if err != nil {
		t.Fatal(err)
	}
	input := string(data)
	got := parse(input)
	want := map[string]map[string]int{
		"light red": {
			"bright white": 1,
			"muted yellow": 2,
		},
		"dark orange": {
			"bright white": 3,
			"muted yellow": 4,
		},
		"bright white": {
			"shiny gold": 1,
		},
		"muted yellow": {
			"shiny gold": 2,
			"faded blue": 9,
		},
		"shiny gold": {
			"dark olive":   1,
			"vibrant plum": 2,
		},
		"dark olive": {
			"faded blue":   3,
			"dotted black": 4,
		},
		"vibrant plum": {
			"faded blue":   5,
			"dotted black": 6,
		},
		"faded blue":   nil,
		"dotted black": nil,
	}
	if !reflect.DeepEqual(got, want) {
		t.Errorf("parse() = %v\nwant: %v", got, want)
	}
}

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.NewFile("example", example1File, "4"),
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
		testcase.NewFile("example1", example1File, "32"),
		testcase.NewFile("example2", example2File, "126"),
		tcPart2,
	} {
		tc.Test(t, Part2)
	}
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
