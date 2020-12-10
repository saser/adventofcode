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
	max := 0
	for _, adapter := range adapters {
		if adapter > max {
			max = adapter
		}
	}
	adapters = append(adapters, 0, max+3)
	sort.Ints(adapters)
	if part == 1 {
		var diffs [4]int
		for i := 0; i < len(adapters)-1; i++ {
			diffs[adapters[i+1]-adapters[i]]++
		}
		return fmt.Sprint(diffs[1] * diffs[3]), nil
	}
	// arrs[i] will contain the number of possible arrangements
	// that includes adapter i.
	// arrs[0] will contain all possible arrangements.
	arrs := make([]int, len(adapters))
	arrs[len(arrs)-1] = 1 // base case
	for i := len(arrs) - 2; i >= 0; i-- {
		sum := 0
		for j := i + 1; j < len(adapters); j++ {
			diff := adapters[j] - adapters[i]
			if diff > 3 {
				break
			}
			sum += arrs[j]
		}
		arrs[i] = sum
	}
	return fmt.Sprint(arrs[0]), nil
}
