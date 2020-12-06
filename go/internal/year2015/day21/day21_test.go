package day21

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/21"

var (
	tcPart1 = testcase.NewFile("input", inputFile, "111")
	tcPart2 = testcase.NewFile("input", inputFile, "188")
)

func Test_playerWins(t *testing.T) {
	player := character{
		hitpoints: 8,
		damage:    5,
		armor:     5,
	}
	boss := character{
		hitpoints: 12,
		damage:    7,
		armor:     2,
	}
	if got, want := playerWins(player, boss), true; got != want {
		t.Errorf("playerWins(%v, %v) = %v; want %v", player, boss, got, want)
	}
}

func TestPart1(t *testing.T) {
	tc := testcase.NewFile(inputFile, inputFile, "111")
	tc.Test(t, Part1)
}

func BenchmarkPart1(b *testing.B) {
	tcPart1.Benchmark(b, Part1)
}

func TestPart2(t *testing.T) {
	tc := testcase.NewFile(inputFile, inputFile, "188")
	tc.Test(t, Part2)
}

func BenchmarkPart2(b *testing.B) {
	tcPart2.Benchmark(b, Part2)
}
