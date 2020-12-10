package day10

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func parse(input string) []int {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	adapters := make([]int, len(lines))
	for i, line := range lines {
		adapter, _ := strconv.Atoi(line)
		adapters[i] = adapter
	}
	return adapters
}

func solve(input string, part int) (string, error) {
	adapters := parse(input)
	sort.Ints(adapters)
	if part == 1 {
		var diffs [4]int
		diffs[adapters[0]]++ // between outlet and first adapter
		diffs[3]++           // between last adapter and device
		for i := 0; i < len(adapters)-1; i++ {
			diffs[adapters[i+1]-adapters[i]]++
		}
		return fmt.Sprint(diffs[1] * diffs[3]), nil
	}
	return "", fmt.Errorf("solution for part %v not implemented", part)
}
