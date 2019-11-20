package day21

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
	"github.com/stretchr/testify/require"
)

const inputFile = "../testdata/21"

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
	require.True(t, playerWins(player, boss))
}

func BenchmarkPart1(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part1)
}
