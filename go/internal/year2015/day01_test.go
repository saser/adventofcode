package year2015

import (
	"io/ioutil"
	"os"
	"strings"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestDay01(t *testing.T) {
	inputFile, err := os.Open("../../../inputs/2015/01")
	require.NoError(t, err)
	inputBytes, err := ioutil.ReadAll(inputFile)
	require.NoError(t, err)
	actual := string(inputBytes)
	t.Run("part1", func(t *testing.T) {
		for _, tt := range []struct {
			name          string
			input, output string
		}{
			{name: "example1_1", input: "(())", output: "0"},
			{name: "example1_2", input: "()()", output: "0"},
			{name: "example2_1", input: "(((", output: "3"},
			{name: "example2_2", input: "(()(()(", output: "3"},
			{name: "example3", input: "))(((((", output: "3"},
			{name: "example4_1", input: "())", output: "-1"},
			{name: "example4_2", input: "))(", output: "-1"},
			{name: "example5_1", input: ")))", output: "-3"},
			{name: "example5_2", input: ")())())", output: "-3"},
			{name: "actual", input: actual, output: "232"},
		} {
			tt := tt
			t.Run(tt.name, func(t *testing.T) {
				answer, err := Day01One(strings.NewReader(tt.input))
				require.NoError(t, err)
				require.Equal(t, tt.output, answer)
			})
		}
	})
}
